"""Adversarial release inventory validation tests."""
import importlib.util
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("release_asset_gate", ROOT / "release_asset_gate.py")
gate = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(gate)


def valid_release():
    version = "0.9.0"
    archives = [
        f"{prefix}-v{version}-{platform}"
        for prefix in ("reprocrate", "reprocrate-desktop")
        for platform in ("linux-x86_64.tar.gz", "macos-arm64.tar.gz", "windows-x86_64.zip")
    ]
    assets = [
        {"name": name, "state": "uploaded", "size": 2048}
        for name in archives
    ] + [
        {"name": f"{name}.sha256", "state": "uploaded", "size": 112}
        for name in archives
    ]
    return {"tag_name": f"v{version}", "draft": True, "prerelease": True, "assets": assets}


class ReleaseAssetGateTests(unittest.TestCase):
    def test_complete_release_passes(self):
        self.assertEqual(gate.verify_release(valid_release(), "0.9.0"), 12)

    def test_missing_extra_and_duplicate_assets_fail(self):
        for edit in ("missing", "extra", "duplicate"):
            candidate = valid_release()
            if edit == "missing":
                candidate["assets"].pop()
            elif edit == "extra":
                candidate["assets"].append({"name": "mystery.zip", "size": 3000, "state": "uploaded"})
            else:
                candidate["assets"][0] = candidate["assets"][1].copy()
            with self.subTest(edit=edit), self.assertRaises(ValueError):
                gate.verify_release(candidate, "0.9.0")

    def test_bad_sizes_and_partial_uploads_fail(self):
        for key, value in [("size", 0), ("state", "starter"), ("size", None)]:
            candidate = valid_release()
            candidate["assets"][0][key] = value
            with self.subTest(key=key, value=value), self.assertRaises(ValueError):
                gate.verify_release(candidate, "0.9.0")
        candidate = valid_release()
        candidate["assets"][-1]["size"] = 2048
        with self.assertRaises(ValueError):
            gate.verify_release(candidate, "0.9.0")

    def test_pre_release_and_source_tag_required(self):
        for key, value in [("draft", False), ("prerelease", False), ("tag_name", "v0.8.0")]:
            candidate = valid_release()
            candidate[key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                gate.verify_release(candidate, "0.9.0")


if __name__ == "__main__":
    unittest.main()
