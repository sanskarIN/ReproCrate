#!/usr/bin/env python3
"""Deterministic dependency inventory from committed lockfiles (not a vulnerability scan)."""
import argparse
import json
import pathlib
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent


def cargo_packages(path):
    with path.open("rb") as stream:
        data = tomllib.load(stream)
    return [
        {
            "ecosystem": "cargo",
            "lockfile": path.relative_to(ROOT).as_posix(),
            "name": p["name"],
            "version": p["version"],
            "source": p.get("source", "workspace"),
            "checksum": p.get("checksum"),
            "license": None,  # Cargo.lock does not contain license metadata.
        }
        for p in data.get("package", [])
    ]


def npm_packages(path):
    data = json.loads(path.read_text(encoding="utf-8"))
    records = []
    for folder, metadata in sorted(data.get("packages", {}).items()):
        if not folder or not folder.startswith("node_modules/"):
            continue
        name = folder.rsplit("node_modules/", 1)[-1]
        if name.startswith("@"):
            # Scoped package name is represented as @scope/name in lockfile path.
            components = folder.split("node_modules/")
            name = components[-1]
        records.append(
            {
                "ecosystem": "npm",
                "lockfile": path.relative_to(ROOT).as_posix(),
                "name": name,
                "version": metadata.get("version", "unknown"),
                "source": metadata.get("resolved", "local"),
                "checksum": metadata.get("integrity"),
                "license": metadata.get("license"),
                "dev": metadata.get("dev", False),
            }
        )
    return records


def build_inventory():
    rust = [ROOT / "Cargo.lock", ROOT / "apps/desktop/src-tauri/Cargo.lock"]
    npm = ROOT / "apps/desktop/package-lock.json"
    for path in [*rust, npm]:
        if not path.is_file():
            raise FileNotFoundError(f"Missing required lockfile: {path}")
    packages = [item for path in rust for item in cargo_packages(path)]
    packages.extend(npm_packages(npm))
    packages.sort(key=lambda x: (x["ecosystem"], x["name"], x["version"], x["lockfile"]))
    return {
        "schema": "reprocrate.dependency-inventory.v1",
        "notice": "Inventory only; not a license clearance, security audit or SBOM attestation.",
        "counts": {
            "cargo": sum(item["ecosystem"] == "cargo" for item in packages),
            "npm": sum(item["ecosystem"] == "npm" for item in packages),
        },
        "packages": packages,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=pathlib.Path, required=True)
    args = parser.parse_args()
    inventory = build_inventory()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("Dependency inventory written (not audited):", args.output)


if __name__ == "__main__":
    main()
