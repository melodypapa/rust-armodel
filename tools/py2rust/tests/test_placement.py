import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from placement import Placement, parse_markdown

MARKDOWN = """\
Table 4.4: Identifiable

| Class        | Identifiable (abstract)   |
|--------------|---------------------------|
| introduction | DocumentationBlock        |
| uuid         | String                    |

| Primitive      | Identifier                     |
|----------------|--------------------------------|
| Package        | M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::PrimitiveTypes |

Table 4.16: AdminData

| Class      | AdminData                                    |
|------------|----------------------------------------------|
| Package    | M2::MSR::AsamHdo::AdminData                  |
| docRevisions | DocRevision                                |

Table 4.1: ARPackage

| Class      | ARPackage (abstract)                         |
|------------|----------------------------------------------|
| Package    | M2::AUTOSARTemplates::GenericStructure::GeneralTemplateClasses::ARPackage |
"""


class TestPlacement(unittest.TestCase):
    def setUp(self):
        self.placement = parse_markdown({"GST.md": MARKDOWN})

    def test_class_binds_to_package_row_in_same_table(self):
        self.assertEqual(
            self.placement.module_of("AdminData"),
            ["m2", "msr", "asam_hdo", "admin_data"],
        )
        self.assertEqual(
            self.placement.module_of("ARPackage"),
            ["m2", "autosar_templates", "generic_structure", "general_template_classes", "ar_package"],
        )

    def test_primitive_binds_too(self):
        self.assertEqual(
            self.placement.module_of("Identifier"),
            ["m2", "autosar_templates", "generic_structure", "general_template_classes", "primitive_types"],
        )

    def test_missing_class_returns_none(self):
        self.assertIsNone(self.placement.module_of("NoSuchClass"))

    def test_snake_segments(self):
        self.assertEqual(Placement.package_to_segments("M2::MSR::AsamHdo::AdminData"),
                         ["m2", "msr", "asam_hdo", "admin_data"])


if __name__ == "__main__":
    unittest.main()
