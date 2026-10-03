"""Regression tests for the P2 model-gap fixes (plan 2026-10-03, Task 0).

Each test names one generated-Rust fact that P2 families need and that the
pre-fix tool got wrong. They parse the generated tree, so run after a
regeneration.
"""
import pathlib

SRC = pathlib.Path(__file__).resolve().parents[3] / "src" / "m2"


def read(rel: str) -> str:
    return (SRC / rel).read_text()


def test_reftyped_fields_use_the_reftype_arena():
    # Unit.physical_dimension_ref must keep BASE/DEST (bytes on the wire),
    # so the field is the arena-linked RefType, not a bare String. (Plan
    # Task 0 originally drafted Option<RefType>-by-value; the arena-link
    # form is what the emitter already supports and what code_guide §6
    # prescribes for referenced classes.)
    units = read("msr/asam_hdo/units.rs")
    assert "physical_dimension_ref: Option<RefTypeId>" in units, (
        "Unit.physical_dimension_ref must be Option<RefTypeId>; "
        "a String drops the DEST attribute and breaks byte round-trip"
    )


def test_base_type_carries_base_type_definition():
    base_types = read("msr/asam_hdo/base_types.rs")
    # Match the field/accessor, not just any mention (compare helpers embed
    # the class name too).
    assert (
        "base_type_definition:" in base_types
        or "pub fn get_base_type_definition" in base_types
    ), (
        "BaseType must expose baseTypeDefinition (py assigns "
        "BaseTypeDirectDefinition() in __init__)"
    )


def test_reference_base_has_its_seven_fields():
    ar_package = read(
        "autosar_templates/generic_structure/general_template_classes/ar_package.rs"
    )
    for field in (
        "short_label",
        "is_default",
        "is_global",
        "base_is_this_package",
        "global_in_package_refs",
        "global_elements",
        "package_ref",
    ):
        assert field in ar_package, f"ReferenceBase is missing {field}"


def test_tref_typed_fields_use_the_tref_arena():
    # TYPE-TREF carries BASE/DEST in the fixtures (batch 2 recon), so the
    # field must be the arena-linked TRefType, not a bare String.
    prototypes = read(
        "autosar_templates/sw_component_template/datatype/data_prototypes.rs"
    )
    assert "type_t_ref: Option<TRefTypeId>" in prototypes, (
        "type_t_ref must be Option<TRefTypeId>; a String drops the "
        "BASE/DEST attributes and breaks byte round-trip"
    )
