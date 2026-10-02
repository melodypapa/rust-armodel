#!/usr/bin/env python3
"""Convert a py-armodel checkout into the rust-armodel model tree.

Usage:
  python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
  python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check
"""
from __future__ import annotations

import argparse
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from emit_document import build_element_variants, emit_document_root, emit_equality, emit_registry
from emit_model import BANNER, emit_module
from extract import extract_repo
from ir import ClassIr, snake_case
from overrides import Overrides
from placement import parse_repo
from templates import (AR_OBJECT_RS, AR_PACKAGE_RS, DOCUMENT_RS, ELEMENT_COLLECTION_RS,
                       IDENTIFIABLE_RS, XML_SPACE_BLOCK)

GTC = "m2/autosar_templates/generic_structure/general_template_classes"

# pinned template -> generated file path (relative to src/); these module leaves
# carry the P0 base chain verbatim and are not produced by emit_module
TEMPLATE_FILES = {
    f"{GTC}/ar_object.rs": AR_OBJECT_RS,
    f"{GTC}/identifiable.rs": IDENTIFIABLE_RS,
    f"{GTC}/element_collection.rs": ELEMENT_COLLECTION_RS,
    f"{GTC}/ar_package.rs": AR_PACKAGE_RS,
}


def py_module_segments(module_py: str) -> list[str]:
    """Fallback placement: py-armodel's file layout mirrors the spec packages
    (`armodel.models.M2.MSR.AsamHdo.AdminData` -> m2/msr/asam_hdo/admin_data)."""
    assert module_py.startswith("armodel.models."), module_py
    return [snake_case(part) for part in module_py.removeprefix("armodel.models.").split(".")]


def group_by_module(ir, overrides, placement):
    """(module segments tuple) -> [ClassIr] plus the count placed via fallback.

    Markdown Package rows are primary; classes the markdown never tables
    (attribute-referenced enums, py helper classes) fall back to their py
    module path, which mirrors the spec packages.
    """
    grouped: dict[tuple[str, ...], list[ClassIr]] = {}
    unplaced: list[str] = []
    fallback = 0
    for cls in ir.classes():
        if cls.is_primitive or not overrides.emits(cls.name):
            continue
        segments = placement.module_of(cls.name)
        if segments is None:
            segments = py_module_segments(cls.module_py)
            fallback += 1
        grouped.setdefault(tuple(segments), []).append(cls)
    return grouped, sorted(unplaced), fallback


def render(ir, overrides: Overrides, placement) -> dict[str, str]:
    """Return {path relative to src/: file content} for the generated tree."""
    files: dict[str, str] = {}
    grouped, _, _ = group_by_module(ir, overrides, placement)

    # template leaves participate in the mod.rs chain but get pinned content
    template_leaves = {tuple(path[: -len(".rs")].split("/")): content
                       for path, content in TEMPLATE_FILES.items()}
    doc_leaf = {tuple(DOCUMENT_RS_PATH[:-len(".rs")].split("/")): None}
    all_segments = list(grouped) + list(template_leaves) + list(doc_leaf)

    dirs = {s[:i] for s in all_segments for i in range(1, len(s))}
    for d in sorted(dirs):
        children = sorted({s[len(d)] for s in all_segments if s[:len(d)] == d and len(s) > len(d)})
        lines = ["//! @generated module chain — do not edit.", ""]
        for child in children:
            lines.append("#[allow(dead_code)]")
            if d == ("m2",) and child == "element_registry":
                lines.append("pub(crate) mod element_registry;")
            else:
                lines.append(f"pub mod {child};")
        files["/".join(d) + "/mod.rs"] = "\n".join(lines) + "\n"

    variants = build_element_variants(ir)
    for segments, classes in sorted(grouped.items()):
        body = emit_module(ir, overrides, placement, classes, list(segments))
        if segments[-1] == "language_data_model":
            body = body.replace(BANNER, BANNER + XML_SPACE_BLOCK + "\n", 1)
        compare_blocks = []
        for cls in sorted(classes, key=lambda c: c.name):
            if cls.is_enum or cls.is_primitive:
                continue
            compare_blocks.append(emit_equality(ir, overrides, cls))
        if compare_blocks:
            body += "\nimpl Document {\n" + "\n\n".join(compare_blocks) + "\n}\n"
        files["/".join(segments) + ".rs"] = body

    files.update(TEMPLATE_FILES)
    files["m2/element_registry.rs"] = emit_registry(ir, overrides, placement, variants)
    files[DOCUMENT_RS_PATH] = emit_document_root(ir, overrides, placement)
    return files


DOCUMENT_RS_PATH = "m2/autosar_templates/autosar_top_level_structure.rs"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--py-armodel", required=True)
    parser.add_argument("--out", default="src")
    parser.add_argument("--check", action="store_true", help="fail (exit 1) if regeneration differs")
    args = parser.parse_args()

    ir = extract_repo(args.py_armodel)
    placement = parse_repo(str(pathlib.Path(args.py_armodel) / "autosar/R23-11/markdown"))
    overrides = Overrides.default()

    grouped, unplaced, fallback = group_by_module(ir, overrides, placement)
    print(f"classes extracted: {len(ir.classes())} (enums: {len(ir.enums())})")
    print(f"emitted classes: {sum(len(v) for v in grouped.values())} "
          f"(markdown-placed: {sum(len(v) for v in grouped.values()) - fallback}, py-module fallback: {fallback})")
    if unplaced:
        print("UNPLACED (add to overrides.skip_emission with a reason, or fix placement):")
        for name in unplaced:
            print(f"  {name}")
        return 2

    out_root = pathlib.Path(args.out)
    changed = 0
    for rel, content in sorted(render(ir, overrides, placement).items()):
        target = out_root / rel
        if args.check:
            if not target.exists() or target.read_text() != content:
                print(f"drift: {rel}")
                changed += 1
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content)
            changed += 1
    from typemap import unknown_types
    if unknown_types():
        print(f"unknown (upstream-placeholder) types mapped to String: {', '.join(unknown_types())}")
    print(f"{'drifted files' if args.check else 'files written'}: {changed}")
    return 1 if (args.check and changed) else 0


if __name__ == "__main__":
    raise SystemExit(main())
