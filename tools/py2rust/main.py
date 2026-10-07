#!/usr/bin/env python3
"""Convert a py-armodel checkout into the rust-armodel model tree.

Usage:
  python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
  python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src --check
"""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))

from emit_document import (build_element_variants, build_arena_names, concrete_classes,
                           emit_document_root, emit_equality, emit_registry)
from emit_model import _module_path_of, emit_module
from extract import extract_repo
from ir import ClassIr
from overrides import Overrides
from placement import parse_repo, py_module_segments
from templates import (AR_OBJECT_RS, AR_PACKAGE_RS, DOCUMENT_RS, ELEMENT_COLLECTION_RS,
                       IDENTIFIABLE_RS, COMPU_REFS_BLOCK, XML_SPACE_BLOCK)

GTC = "m2/autosar_templates/generic_structure/general_template_classes"

# pinned template -> generated file path (relative to src/); these module leaves
# carry the P0 base chain verbatim and are not produced by emit_module
TEMPLATE_FILES = {
    f"{GTC}/ar_object.rs": AR_OBJECT_RS,
    f"{GTC}/identifiable.rs": IDENTIFIABLE_RS,
    f"{GTC}/element_collection.rs": ELEMENT_COLLECTION_RS,
    f"{GTC}/ar_package.rs": AR_PACKAGE_RS,
}

# imports the pinned templates need: {path: [(use-path, name), ...]}; the merge
# strips the templates' own use lines and re-adds these per name, skipping any
# name the generated body already imports or the template defines itself.
# The ar_object.rs entry (one import per ElementRef variant) is computed in
# render() and added to this table.
GTC_MODULE = GTC.replace("/", "::")
_C = f"crate::{GTC_MODULE}"

TEMPLATE_EXTRA_IMPORTS = {
    f"{GTC}/identifiable.rs": [
        (f"{_C}::ar_object", "ARObject"),
        (f"{_C}::ar_object", "ElementRef"),
        ("crate::m2::msr::asam_hdo::admin_data", "AdminDataId"),
        ("crate::m2::msr::documentation::annotation", "AnnotationId"),
        ("crate::m2::msr::documentation::text_model::block_elements", "DocumentationBlockId"),
        ("crate::m2::msr::documentation::text_model::multilanguage_data", "MultiLanguageOverviewParagraphId"),
    ],
    f"{GTC}/element_collection.rs": [
        (f"{_C}::identifiable", "Identifiable"),
    ],
    f"{GTC}/ar_package.rs": [
        (f"{_C}::ar_object", "ARObject"),
        (f"{_C}::ar_object", "ElementRef"),
        (f"{_C}::element_collection", "CollectableElement"),
        ("crate::m2::msr::asam_hdo::admin_data", "AdminDataId"),
        # the pinned ReferenceBase stores RefTypes by arena key
        (f"{_C}::primitive_types", "RefTypeId"),
        # the template's new_key_type! block needs it when the leaf has no
        # generated group body that already imports it
        ("slotmap", "new_key_type"),
    ],
}



def _mod_rs(children: list[str], body: str) -> str:
    """mod.rs = body with its leading //! doc block kept first, then the pub mod items."""
    lines = body.splitlines()
    i = 0
    while i < len(lines) and (lines[i].startswith("//!") or lines[i].strip() == "" or lines[i].startswith("// @generated")):
        i += 1
    mods: list[str] = []
    for c in children:
        mods.append("#[allow(dead_code)]")
        mods.append(f"pub mod {c};")
    return "\n".join(lines[:i] + mods + lines[i:]) + "\n"

# pinned template classes that still need a generated compare (their IR fields
# drive it); ARObject/ARPackage keep their hand-written P0 compares in the root
TEMPLATE_COMPARE_CLASSES = {
    f"{GTC}/identifiable.rs": ["Referrable", "MultilanguageReferrable", "Identifiable"],
    f"{GTC}/element_collection.rs": ["CollectableElement"],
    f"{GTC}/ar_package.rs": ["PackageableElement", "ARElement", "ReferenceBase"],
    f"{GTC}/ar_object.rs": [],
}


def _merge_template(generated: str, template: str, path: str,
                    extra_imports: dict[str, list[tuple[str, str]]]) -> str:
    """Generated group body + pinned template structs. The template's own use
    lines and //! comments are dropped (duplicates, or illegal mid-file //!);
    the imports the templates need are re-added per name, skipping any name the
    generated body already imports or the template defines itself."""
    body_lines = [l for l in template.splitlines()
                  if not l.startswith("use ") and not l.startswith("//!")
                  and not l.startswith("// @generated")]
    imported: set[str] = set()
    for line in generated.splitlines():
        s = line.strip()
        if s.startswith("use ") and s.endswith(";"):
            last = s[4:-1].rsplit("::", 1)[-1].strip("{} ")
            imported.update(n.strip() for n in last.split(",") if n.strip())
    defined = set(re.findall(r"\b(?:struct|enum|type)\s+([A-Z]\w*)", template))
    extras = [f"use {module}::{name};"
              for module, name in extra_imports.get(path, [])
              if name not in imported and name not in defined]
    parts = [generated] if generated else []
    if extras:
        parts.append("\n".join(extras))
    parts.append("\n".join(body_lines))
    return "\n".join(parts) + "\n"


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
    # the registry file is emitted directly (no class group) but must still
    # appear in m2's mod.rs chain (pub(crate))
    all_segments = (list(grouped) + list(template_leaves) + list(doc_leaf)
                    + [("m2", "element_registry")])

    def has_deeper(s):
        return any(o[:len(s)] == s and len(o) > len(s) for o in all_segments)

    variants = build_element_variants(ir, overrides)
    arena_names = build_arena_names(concrete_classes(ir, overrides))
    for segments, classes in sorted(grouped.items()):
        body = emit_module(ir, overrides, placement, classes, list(segments))
        if segments[-1] in ("language_data_model", "computation_method"):
            # inner doc comments must precede every item (E0753): insert the
            # pinned block after the //! header, before the first use line
            block = (XML_SPACE_BLOCK
                     if segments[-1] == "language_data_model" else COMPU_REFS_BLOCK)
            lines = body.splitlines()
            for i, line in enumerate(lines):
                if line.startswith("use "):
                    lines.insert(i, block)
                    break
            body = "\n".join(lines)
        compare_blocks = []
        for cls in sorted(classes, key=lambda c: c.name):
            if cls.is_enum or cls.is_primitive:
                continue
            compare_blocks.append(emit_equality(ir, overrides, cls, None, arena_names))
        if compare_blocks:
            body += "\nimpl Document {\n" + "\n\n".join(compare_blocks) + "\n}\n"
        if has_deeper(segments):
            # group is also a parent dir: its classes live in mod.rs next to pub mod lines
            children = sorted({o[len(segments)] for o in all_segments
                               if o[:len(segments)] == segments and len(o) > len(segments)})
            files["/".join(segments) + "/mod.rs"] = _mod_rs(children, body)
        else:
            files["/".join(segments) + ".rs"] = body

    # plain directories without their own group get a declarative mod.rs
    dirs = {s[:i] for s in all_segments for i in range(1, len(s))}
    for d in sorted(dirs):
        path = "/".join(d) + "/mod.rs"
        if path in files:
            continue
        children = sorted({s[len(d)] for s in all_segments if s[:len(d)] == d and len(s) > len(d)})
        lines = ["//! @generated module chain — do not edit.", ""]
        for child in children:
            if d == ("m2",) and child == "element_registry":
                # the registry file carries its own #![allow(dead_code)];
                # duplicating the attr here trips clippy's duplicated_attributes
                lines.append("pub(crate) mod element_registry;")
            else:
                lines.append("#[allow(dead_code)]")
                lines.append(f"pub mod {child};")
        files[path] = "\n".join(lines) + "\n"

    # pinned template leaves: merged with any generated group at the same path
    element_variants = "\n".join(f"    {name}({name}Id)," for name in variants
                                 if name != "ARPackage")  # the template carries it
    template_leaves = {k: v.replace("    // %%ELEMENT_VARIANTS%% — appended by tools/py2rust from ARPackage's create-directory", element_variants)
                       for k, v in template_leaves.items()}

    # the pinned ElementRef enum needs one import per variant's Id type
    extra_imports = {path: list(items) for path, items in TEMPLATE_EXTRA_IMPORTS.items()}
    ar_object_extras = [(f"crate::{GTC_MODULE}::ar_package", "ARPackageId")]
    for name in variants:
        if name == "ARPackage":
            continue  # the template imports ARPackageId itself
        module = _module_path_of(name, ir, overrides, placement)
        if module:
            ar_object_extras.append((f"crate::{module}", f"{name}Id"))
    extra_imports[f"{GTC}/ar_object.rs"] = ar_object_extras

    for leaf_segments, template in template_leaves.items():
        base_name = "/".join(leaf_segments) + ".rs"
        group = grouped.get(leaf_segments)
        body = ""
        compare_blocks = []
        if group is not None:
            body = emit_module(ir, overrides, placement, group, list(leaf_segments))
            for cls in sorted(group, key=lambda c: c.name):
                if cls.is_enum or cls.is_primitive:
                    continue
                compare_blocks.append(emit_equality(ir, overrides, cls, None, arena_names))
        # template-carried structs only compare what they carry: restrict each
        # compare to the methods the pinned template actually defines
        available = set(re.findall(r"fn (\w+)\(", template))
        for name in TEMPLATE_COMPARE_CLASSES.get(base_name, []):
            cls = ir.get(name)
            if cls is not None:
                compare_blocks.append(emit_equality(ir, overrides, cls, available, arena_names))
        if compare_blocks:
            body += "\nimpl Document {\n" + "\n\n".join(compare_blocks) + "\n}\n"
        merged = _merge_template(body, template, base_name, extra_imports)
        if has_deeper(leaf_segments):
            leaf_path = "/".join(leaf_segments) + "/mod.rs"
            children = sorted({o[len(leaf_segments)] for o in all_segments
                               if o[:len(leaf_segments)] == leaf_segments and len(o) > len(leaf_segments)})
            files[leaf_path] = _mod_rs(children, merged)
        else:
            files[base_name] = merged
    files["m2/element_registry.rs"] = emit_registry(ir, overrides, placement, variants)
    # derives the identical arena_names from concrete_classes deterministically
    files[DOCUMENT_RS_PATH] = emit_document_root(ir, overrides, placement)
    return files


DOCUMENT_RS_PATH = "m2/autosar_templates/autosar_top_level_structure.rs"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--py-armodel", required=True)
    parser.add_argument("--out", default="src")
    parser.add_argument("--check", action="store_true", help="fail (exit 1) if regeneration differs")
    parser.add_argument("--emit-port-checklist", action="store_true",
                        help="write docs/port_checklist.md (py read*/write* methods, ported status by scanning src/)")
    parser.add_argument("--emit-dispatch-tables", action="store_true",
                        help="emit reader/writer dispatch tables for ported handlers (P5)")
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
    # stage the generated tree, rustfmt it, then copy/compare — the emitter
    # produces rustfmt-clean code only approximately (import order, long
    # base chains), so formatting is part of the pipeline, not a hand edit
    files_map = render(ir, overrides, placement)
    with tempfile.TemporaryDirectory() as tmp:
        staged = pathlib.Path(tmp)
        for rel, content in sorted(files_map.items()):
            target = staged / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(content)
        subprocess.run(
            ["rustfmt", "--edition", "2021", *(str(p) for p in sorted(staged.rglob("*.rs")))],
            check=True)
        for rel in sorted(files_map):
            gen = (staged / rel).read_text()
            target = out_root / rel
            if args.check:
                if not target.exists() or target.read_text() != gen:
                    print(f"drift: {rel}")
                    changed += 1
            else:
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(gen)
                changed += 1
                if rel.endswith("/mod.rs"):
                    stale = out_root / (rel[:-len("mod.rs")] + ".rs")
                    if stale.exists():
                        stale.unlink()
    from typemap import unknown_types
    if unknown_types():
        print(f"unknown (upstream-placeholder) types mapped to String: {', '.join(unknown_types())}")
    print(f"{'drifted files' if args.check else 'files written'}: {changed}")
    if args.emit_port_checklist:
        # flat sibling import, matching main.py's other imports (ir, typemap,
        # …) — running as a script puts tools/py2rust on sys.path, not tools/
        import port_checklist as port_checklist_mod

        methods = port_checklist_mod.collect_py_methods(
            pathlib.Path(args.py_armodel) / "src" / "armodel")
        doc_root = out_root.parent / "docs"
        doc_root.mkdir(parents=True, exist_ok=True)
        (doc_root / "port_checklist.md").write_text(
            port_checklist_mod.render(methods, out_root))
        fns = port_checklist_mod.collect_fn_names(out_root)
        ported = sum(1 for name, _ in methods
                     if port_checklist_mod.rust_name(name) in fns)
        print(f"port checklist: {ported}/{len(methods)} reader/writer methods ported")
    if args.emit_dispatch_tables:
        # flat sibling import, matching main.py's other imports (see above)
        import dispatch_tables as dispatch_tables_mod

        reader_table, writer_table = dispatch_tables_mod.emit_dispatch_tables(out_root)
        print(f"dispatch tables: {reader_table}, {writer_table}")
    return 1 if (args.check and changed) else 0


if __name__ == "__main__":
    raise SystemExit(main())
