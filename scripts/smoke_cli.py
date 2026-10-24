#!/usr/bin/env python3
"""Headless smoke tests against the ACTUAL built ReproCrate CLI executable.

Only exercises inert/offline commands; does not launch user-selected workloads.
This is not a desktop GUI or clean-machine smoke test.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib


def run(binary, *args, succeed=True):
    result = subprocess.run(
        [str(binary), *map(str, args)],
        capture_output=True,
        text=True,
        timeout=30,
        check=False,
    )
    if (result.returncode == 0) != succeed:
        raise AssertionError(
            f"Unexpected result: {args}; rc={result.returncode}; stderr={result.stderr[:500]}"
        )
    return result.stdout


def check(binary):
    version = run(binary, "--version")
    manifest = Path(__file__).resolve().parents[1] / "Cargo.toml"
    with manifest.open("rb") as stream:
        expected_version = tomllib.load(stream)["workspace"]["package"]["version"]
    if version.strip() != f"reprocrate {expected_version}":
        raise AssertionError(f"Unexpected CLI version: {version[:100]}")
    run(binary, "--help")
    env = json.loads(run(binary, "doctor", "--json"))
    if not all(isinstance(env.get(key), str) and env[key] for key in ("os", "architecture", "capture_profile")):
        raise AssertionError("Missing basic environment fields")
    if any(field in env for field in ("hostname", "username", "ip_address", "env")):
        raise AssertionError("Unexpected identifier in environment response")

    with tempfile.TemporaryDirectory(prefix="reprocrate-smoke-") as folder:
        directory = Path(folder)
        archive = directory / "preview.zip"
        checksum = directory / "preview.zip.sha256"
        content = b"ReproCrate offline QA fixture, never executed or extracted."
        archive.write_bytes(content)
        checksum.write_text(f"{hashlib.sha256(content).hexdigest()}  preview.zip\n", encoding="utf-8")
        output = run(binary, "verify-download", archive, "--checksum", checksum)
        if "SHA-256 matches" not in output:
            raise AssertionError("Archive verification did not succeed")
        archive.write_bytes(content + b"tampered")
        run(binary, "verify-download", archive, "--checksum", checksum, succeed=False)

        fixture = directory / "build.log"
        analysis = directory / "analysis.json"
        fixture.write_text("error: permission denied\n", encoding="utf-8")
        run(binary, "analyze", fixture, "--output", analysis)
        parsed = json.loads(analysis.read_text(encoding="utf-8"))
        if not parsed.get("findings"):
            raise AssertionError("No error finding from known test fixture")

        run(binary, "analyze", fixture, "--output", analysis, succeed=False)
    print("Offline release smoke passed: CLI version, doctor, checksum, corruption, log analysis and overwrite prevention.")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    args = parser.parse_args()
    if not args.binary.is_file():
        print(f"Missing binary: {args.binary}", file=sys.stderr)
        return 2
    check(args.binary.resolve())
    return 0


if __name__ == "__main__":
    sys.exit(main())
