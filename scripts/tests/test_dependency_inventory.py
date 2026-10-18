"""Inventory generator smoke and regression checks."""
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "dependency_inventory.py"
SPEC = importlib.util.spec_from_file_location("dependency_inventory", SCRIPT)
inventory_module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(inventory_module)


class InventoryTests(unittest.TestCase):
    def test_all_locked_sources_included_without_claiming_audit(self):
        record = inventory_module.build_inventory()
        self.assertEqual(record["schema"], "reprocrate.dependency-inventory.v1")
        self.assertGreater(record["counts"]["cargo"], 0)
        self.assertGreater(record["counts"]["npm"], 0)
        self.assertIn("not a license clearance", record["notice"])
        names = {x["name"] for x in record["packages"]}
        self.assertIn("reprocrate-core", names)
        self.assertIn("reprocrate-desktop", names)
        self.assertIn("react", names)

    def test_locked_package_versions_and_source_are_preserved(self):
        data = inventory_module.build_inventory()
        rust = [p for p in data["packages"] if p["ecosystem"] == "cargo"]
        self.assertTrue(all(p["name"] and p["version"] for p in rust))
        self.assertTrue(all(p["license"] is None for p in rust))
        npm = [p for p in data["packages"] if p["ecosystem"] == "npm"]
        self.assertTrue(all(p["name"] and p["version"] for p in npm))

    def test_stable_order_and_json_encoding(self):
        a = inventory_module.build_inventory()
        b = inventory_module.build_inventory()
        self.assertEqual(a, b)
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / "inventory.json"
            output.write_text(json.dumps(a, sort_keys=True), encoding="utf-8")
            self.assertEqual(json.loads(output.read_text(encoding="utf-8"))["counts"], a["counts"])


if __name__ == "__main__":
    unittest.main()
