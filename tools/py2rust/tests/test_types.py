import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from ir import ClassIr, FieldIr, Ir
from overrides import Overrides
from typemap import RustField, map_field


def build_ir() -> Ir:
    ir = Ir()
    ir.add(ClassIr(name="AdminData", module_py="m", bases=["ARObject"]))
    ir.add(ClassIr(name="Sdg", module_py="m", bases=["ARObject"]))
    ir.add(ClassIr(name="BindingTimeEnum", module_py="m", bases=["AREnum"], is_enum=True))
    return ir


class TestMapField(unittest.TestCase):
    def setUp(self):
        self.ir = build_ir()
        self.overrides = Overrides.default()

    def test_primitive_maps_to_string(self):
        field = FieldIr(name="gid", type_expr="Optional[NameToken]", kind="optional", inner="NameToken")
        self.assertEqual(map_field(field, self.ir, self.overrides),
                         RustField(rust_name="gid", rust_type="String", wrapper="optional", link=None))

    def test_enum_field_uses_alias(self):
        field = FieldIr(name="bindingTime", type_expr="Optional[BindingTimeEnum]",
                        kind="optional", inner="BindingTimeEnum")
        mapped = map_field(field, self.ir, self.overrides)
        self.assertEqual(mapped.rust_type, "BindingTimeEnum")

    def test_aliased_enum_maps_to_hand_written_type(self):
        self.ir.add(ClassIr(name="XmlSpaceEnum", module_py="m", bases=["AREnum"], is_enum=True))
        field = FieldIr(name="xmlSpace", type_expr="Optional[XmlSpaceEnum]", kind="optional", inner="XmlSpaceEnum")
        mapped = map_field(field, self.ir, self.overrides)
        self.assertEqual(mapped.rust_type, "XmlSpace")  # aliased to the hand-written P0 enum

    def test_model_field_is_arena_link(self):
        field = FieldIr(name="sdgs", type_expr="List[Sdg]", kind="list", inner="Sdg")
        mapped = map_field(field, self.ir, self.overrides)
        self.assertEqual(mapped.rust_type, "SdgId")
        self.assertEqual(mapped.wrapper, "list")
        self.assertEqual(mapped.link, "Sdg")

    def test_unknown_inner_maps_to_string_and_is_reported(self):
        # py-armodel ships placeholder references (e.g. V2xSupportEnum) that are
        # never defined — mirror them as String and surface them in the report
        import typemap
        typemap.UNKNOWN_LOG.clear()
        field = FieldIr(name="weird", type_expr="Optional[Mystery]", kind="optional", inner="Mystery")
        mapped = map_field(field, self.ir, self.overrides)
        self.assertEqual(mapped.rust_type, "String")
        self.assertEqual(typemap.unknown_types(), ["Mystery"])
