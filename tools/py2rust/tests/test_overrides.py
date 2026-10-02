import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from overrides import Overrides


class TestOverrides(unittest.TestCase):
    def test_default_aliases(self):
        overrides = Overrides.default()
        self.assertEqual(overrides.type_alias("XmlSpaceEnum"), "XmlSpace")
        self.assertIsNone(overrides.type_alias("Sdg"))

    def test_skip_emission_covers_templates_and_meta_bases(self):
        overrides = Overrides.default()
        for name in ("XmlSpaceEnum", "ARObject", "Referrable", "Identifiable", "ARPackage",
                     "CollectableElement", "AUTOSAR", "AbstractAUTOSAR", "AUTOSARDoc",
                     "FileInfoComment", "ARType", "ARLiteral", "AREnum"):
            self.assertIn(name, overrides.skip_emission, name)

    def test_tag_overrides_win_over_kebab_rule(self):
        overrides = Overrides.default()
        overrides.tag_overrides["IEEE1722TpConnection"] = "IEEE1722TP-CONNECTION"
        self.assertEqual(overrides.tag_for("IEEE1722TpConnection"), "IEEE1722TP-CONNECTION")
        self.assertEqual(overrides.tag_for("ARPackage"), "AR-PACKAGE")  # kebab fallback


if __name__ == "__main__":
    unittest.main()
