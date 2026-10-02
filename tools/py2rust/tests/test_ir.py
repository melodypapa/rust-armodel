import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from ir import ClassIr, FieldIr, Ir


class TestIr(unittest.TestCase):
    def test_field_rust_name_snake_cases_camel(self):
        self.assertEqual(FieldIr.rust_name_for("usedLanguages"), "used_languages")
        self.assertEqual(FieldIr.rust_name_for("DocRevisions"), "doc_revisions")
        self.assertEqual(FieldIr.rust_name_for("gid"), "gid")
        self.assertEqual(FieldIr.rust_name_for("xmlSpace"), "xml_space")

    def test_ir_registers_classes_and_enums(self):
        ir = Ir()
        ir.add(ClassIr(name="Sd", module_py="armodel.models.M2.X", bases=["ARObject"]))
        ir.add(ClassIr(name="XmlSpaceEnum", module_py="armodel.models.M2.Y", bases=["AREnum"], is_enum=True))
        self.assertEqual({c.name for c in ir.classes()}, {"Sd", "XmlSpaceEnum"})
        self.assertEqual([c.name for c in ir.enums()], ["XmlSpaceEnum"])


if __name__ == "__main__":
    unittest.main()
