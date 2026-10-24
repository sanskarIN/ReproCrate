"""v0.9.1 release metadata and RustSec remediation regression tests."""
import json
from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]


def read_toml(path):
    with (ROOT / path).open("rb") as stream:
        return tomllib.load(stream)


class VersionAndLockfileTests(unittest.TestCase):
    def test_all_release_versions_agree(self):
        self.assertEqual(read_toml("Cargo.toml")["workspace"]["package"]["version"], "0.9.1")
        self.assertEqual(read_toml("apps/desktop/src-tauri/Cargo.toml")["package"]["version"], "0.9.1")
        package = json.loads((ROOT / "apps/desktop/package.json").read_text())
        package_lock = json.loads((ROOT / "apps/desktop/package-lock.json").read_text())
        tauri = json.loads((ROOT / "apps/desktop/src-tauri/tauri.conf.json").read_text())
        for value in (package["version"], package_lock["version"],
                      package_lock["packages"][""]["version"], tauri["version"]):
            self.assertEqual(value, "0.9.1")

    def test_zip_is_patched_in_both_rust_lockfiles(self):
        for path in ("Cargo.lock", "apps/desktop/src-tauri/Cargo.lock"):
            with self.subTest(path=path):
                lock = read_toml(path)
                versions = [p["version"] for p in lock["package"] if p["name"] == "zip"]
                self.assertEqual(versions, ["9.0.5"])
        workspace = read_toml("Cargo.toml")
        self.assertEqual(workspace["workspace"]["package"]["rust-version"], "1.88")
        self.assertEqual(workspace["workspace"]["dependencies"]["zip"]["version"], "=9.0.5")

    def test_local_crates_match_release(self):
        expected = {"reprocrate-cli": "0.9.1", "reprocrate-core": "0.9.1"}
        packages = {p["name"]: p["version"] for p in read_toml("Cargo.lock")["package"]}
        for name, version in expected.items():
            self.assertEqual(packages[name], version)
        desktop_packages = {p["name"]: p["version"]
                            for p in read_toml("apps/desktop/src-tauri/Cargo.lock")["package"]}
        self.assertEqual(desktop_packages["reprocrate-core"], "0.9.1")
        self.assertEqual(desktop_packages["reprocrate-desktop"], "0.9.1")


if __name__ == "__main__":
    unittest.main()
