#!/usr/bin/env python3
"""Check GitHub Releases API metadata before making a draft public.

This checks expected assets, sizes and upload states, not their publisher
authenticity or byte-level hashes. The release runners verify hashes locally.
"""
import argparse
import json
import sys


def verify_release(release, version):
    if release.get("tag_name") != f"v{version}":
        raise ValueError("Unexpected release tag")
    if release.get("draft") is not True:
        raise ValueError("Expected unpublished draft release")
    if release.get("prerelease") is not True:
        raise ValueError("Release must be marked as a prerelease")

    suffixes = ("linux-x86_64.tar.gz", "macos-arm64.tar.gz", "windows-x86_64.zip")
    archives = {
        f"{prefix}-v{version}-{suffix}"
        for prefix in ("reprocrate", "reprocrate-desktop")
        for suffix in suffixes
    }
    expected = archives | {f"{name}.sha256" for name in archives}
    assets = release.get("assets")
    if not isinstance(assets, list):
        raise ValueError("Missing release assets")

    names = [asset.get("name") for asset in assets if isinstance(asset, dict)]
    if len(names) != len(expected) or set(names) != expected:
        missing = sorted(expected - set(names))
        unexpected = sorted(set(names) - expected)
        raise ValueError(f"Incorrect release assets: missing={missing}; unexpected={unexpected}")

    for asset in assets:
        name = asset["name"]
        size = asset.get("size")
        if asset.get("state") != "uploaded":
            raise ValueError(f"Asset is not uploaded: {name}")
        if not isinstance(size, int) or isinstance(size, bool):
            raise ValueError(f"Asset size missing: {name}")
        if name in archives and size < 1024:
            raise ValueError(f"Executable archive is implausibly small: {name}")
        if name not in archives and not 80 <= size <= 512:
            raise ValueError(f"SHA-256 sidecar has invalid size: {name}")
    return len(assets)


def select_release(source, version):
    """GitHub does not expose unpublished drafts at the tag lookup endpoint.

    Enumerate accessible releases, requiring exactly one matching tag instead.
    """
    if not isinstance(source, list):
        return source
    matches = [
        entry for entry in source
        if isinstance(entry, dict) and entry.get("tag_name") == f"v{version}"
    ]
    if len(matches) != 1:
        raise ValueError("Expected exactly one draft with the requested tag")
    return matches[0]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    args = parser.parse_args()
    try:
        source = select_release(json.load(sys.stdin), args.version)
        count = verify_release(source, args.version)
    except (ValueError, TypeError, json.JSONDecodeError) as error:
        print(f"Release integrity gate failed: {error}", file=sys.stderr)
        return 1
    print(f"Release metadata gate passed: {count} distinct uploaded assets")
    return 0


if __name__ == "__main__":
    sys.exit(main())
