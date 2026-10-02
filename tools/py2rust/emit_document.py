"""ElementRef variants, tag→constructor registry, generated equality, Document root.

The Document root (templates.DOCUMENT_RS) is surgically extended: arenas are
regenerated for every concrete class, P0's per-class resolvers and the P0
compares that move to generated modules are removed, and a uniform
resolver/factory impl block is appended. P0's root accessors, add_ar_package,
add_element, assert_structurally_equal skeleton, compare_ar_object/compare_ar_package
and the #[cfg(test)] module are kept verbatim.
"""
from __future__ import annotations

import templates
from ir import ClassIr, Ir, snake_case
from overrides import Overrides
from placement import Placement
from emit_model import method_stem
from typemap import map_field

# P0 compares that move to the generated per-module impl blocks
REMOVED_COMPARES = [
    "compare_admin_data",
    "compare_multi_language_plain_text",
    "compare_sdg",
    "compare_sdg_contents",
]

ARENAS_MARKER = "// — arenas (one per concrete class) —"
ROOT_FIELDS_MARKER = "// — root fields (spec class `AUTOSAR`) —"
RESOLVERS_MARKER = "// — id resolvers (read access from outside the crate) —"
EQUALITY_MARKER = "// — structural equality (P0 design §7) —"
TESTS_MARKER = "#[cfg(test)]"


def plural(name: str) -> str:
    snake = snake_case(name)
    # P0 arena names never double the trailing s (SdgContents -> sdg_contents)
    return snake if snake.endswith("s") else snake + "s"


def concrete_classes(ir: Ir, overrides: Overrides) -> list[ClassIr]:
    """Every class that gets a Document arena: emitted concrete classes plus
    ARPackage/ReferenceBase, whose structs are pinned in the ar_package.rs
    template (skip_emission stops struct emission only) but whose arenas,
    resolvers and the hand-written add_ar_package the root still needs."""
    concrete = [
        c for c in ir.classes()
        if not c.is_abstract and not c.is_enum and not c.is_primitive
        and overrides.emits(c.name)
    ]
    present = {c.name for c in concrete}
    for name in ("ARPackage", "ReferenceBase"):
        if name not in present:
            cls = ir.get(name)
            if cls is not None:
                concrete.append(cls)
    concrete.sort(key=lambda c: c.name)
    return concrete


def build_arena_names(concrete: list[ClassIr]) -> dict[str, str]:
    """class name -> Document arena field name. plural() is not injective
    (CompuScale and CompuScales both map to compu_scales), so later collisions
    get a deterministic _arena suffix."""
    names: dict[str, str] = {}
    taken: set[str] = set()
    for c in concrete:  # sorted by name
        field = plural(c.name)
        if field in taken:
            field = f"{field}_arena"
        names[c.name] = field
        taken.add(field)
    return names


def build_element_variants(ir: Ir, overrides: Overrides) -> list[str]:
    """ARPackage first, then every class ARPackage's createXxx directory constructs."""
    variants = ["ARPackage"]
    for _factory, target in sorted(ir.get("ARPackage").creates.items()):
        cls = ir.get(target)
        if (target not in variants and cls and not cls.is_abstract
                and overrides.emits(target)):
            variants.append(target)
    return variants


# The common Identifiable parts the parser reads and the writer emits on
# AR-PACKAGE/ELEMENTS children: (rust getter, py field, settable by the parser).
# Capability per variant = the py field's defining class appears in the
# variant's base chain (the pinned templates own these fields). py stores
# `shortNameFragments`, so short_name has no defining field — it is
# template-owned and every struct carries it.
COMMON_PARTS = [
    ("get_short_name", None, False),
    ("get_uuid", "uuid", True),
    ("get_category", "category", True),
    ("get_checksum", "checksum", True),
    ("get_timestamp", "timestamp", True),
]

# error-message labels in compare_element (AUTOSAR tag spelling)
_COMMON_LABELS = {
    "short_name": "SHORT-NAME",
    "uuid": "UUID",
    "category": "CATEGORY",
    "checksum": "CHECKSUM",
    "timestamp": "TIMESTAMP",
}


def _base_closure(ir: Ir, name: str) -> set[str]:
    """Transitive base classes of `name` (bases may be mixins, walk all of them)."""
    out: set[str] = set()
    cls = ir.get(name)
    stack = list(cls.bases) if cls is not None else []
    while stack:
        cur = stack.pop()
        if cur in out:
            continue
        out.add(cur)
        base = ir.get(cur)
        if base is not None:
            stack.extend(base.bases)
    return out


def _variant_capabilities(ir: Ir, variants: list[str]) -> dict[str, set[str]]:
    """variant name -> the COMMON_PARTS getters its struct actually has."""
    definers = {
        py_field: {c.name for c in ir.classes()
                   if any(f.name == py_field for f in c.fields)}
        for _getter, py_field, _settable in COMMON_PARTS if py_field is not None
    }
    caps: dict[str, set[str]] = {}
    for name in variants:
        chain = _base_closure(ir, name)
        caps[name] = {getter for getter, py_field, _ in COMMON_PARTS
                      if py_field is None or definers[py_field] & chain}
    return caps


def emit_registry(ir: Ir, overrides: Overrides, placement: Placement, variants: list[str]) -> str:
    ar_object = overrides.alias_module["ElementRef"]
    ar_package = "::".join(placement.module_of("ARPackage"))
    arena_names = build_arena_names(concrete_classes(ir, overrides))
    caps = _variant_capabilities(ir, variants)

    lines = [
        "// @generated by tools/py2rust — tag → constructor registry plus the",
        "// element-kind-generic helpers over the common Identifiable parts.",
        "// Do not edit.",
        "#![allow(dead_code)]",
        "use crate::Document;",
        f"use crate::{ar_object}::ElementRef;",
        f"use crate::{ar_package}::ARPackageId;",
    ]
    lines += [
        "",
        "pub(crate) type ElementFactory =",
        "    fn(&mut Document, Option<ARPackageId>, &str) -> ElementRef;",
        "",
        "pub(crate) fn element_factory_for_tag(tag: &str) -> Option<ElementFactory> {",
        "    match tag {",
    ]
    for name in variants:
        add = "add_ar_package" if name == "ARPackage" else f"add_{snake_case(name)}"
        lines += [
            f'        "{overrides.tag_for(name)}" => Some(|d: &mut Document, p: Option<ARPackageId>, n: &str| {{',
            f"            ElementRef::{name}(d.{add}(p, n))",
            "        }),",
        ]
    lines += ["        _ => None,", "    }", "}", ""]

    # element_tag: the AUTOSAR tag spelling of each variant
    lines += [
        "pub(crate) fn element_tag(element: &ElementRef) -> &'static str {",
        "    match element {",
    ]
    for name in variants:
        lines.append(f'        ElementRef::{name}(_) => "{overrides.tag_for(name)}",')
    lines += ["    }", "}", ""]

    # one getter per common part (+ setter where the parser reads it);
    # variants whose base chain lacks the part fall to the wildcard
    for getter, _py_field, settable in COMMON_PARTS:
        field = getter[len("get_"):]
        capable = [name for name in variants if getter in caps[name]]
        lines += [
            f"pub(crate) fn element_{field}<'a>(",
            "    d: &'a Document,",
            "    element: &ElementRef,",
            ") -> Option<&'a str> {",
            "    match element {",
        ]
        for name in capable:
            arena = arena_names[name]
            lines.append(
                f"        ElementRef::{name}(id) => d.{arena}.get(*id).and_then(|e| e.{getter}()),")
        if len(capable) < len(variants):
            lines.append("        _ => None,")
        lines += ["    }", "}", ""]
        if settable:
            lines += [
                f"pub(crate) fn element_set_{field}(",
                "    d: &mut Document,",
                "    element: &ElementRef,",
                "    value: &str,",
                ") {",
                "    match element {",
            ]
            for name in capable:
                arena = arena_names[name]
                lines += [
                    f"        ElementRef::{name}(id) => {{",
                    f"            if let Some(e) = d.{arena}.get_mut(*id) {{",
                    f"                e.set_{field}(value);",
                    "            }",
                    "        }",
                ]
            if len(capable) < len(variants):
                lines.append("        _ => {}")
            lines += ["    }", "}", ""]

    # kind + common-part comparison for the P0 package compare
    lines += [
        "pub(crate) fn compare_element(",
        "    d: &Document,",
        "    other: &Document,",
        "    a: &ElementRef,",
        "    b: &ElementRef,",
        "    path: &str,",
        ") -> Result<(), String> {",
        "    if std::mem::discriminant(a) != std::mem::discriminant(b) {",
        '        return Err(format!("{path}: element kind mismatch"));',
        "    }",
        "    match (a, b) {",
    ]
    for name in variants:
        arena = arena_names[name]
        lines += [
            f"        (ElementRef::{name}(x), ElementRef::{name}(y)) => {{",
            f"            let ea = d.{arena}.get(*x)",
            '                .ok_or_else(|| format!("{path}: id not found in own arena"))?;',
            f"            let eb = other.{arena}.get(*y)",
            '                .ok_or_else(|| format!("{path}: id not found in other arena"))?;',
        ]
        for getter, _py_field, _settable in COMMON_PARTS:
            if getter not in caps[name]:
                continue
            field = getter[len("get_"):]
            lines += [
                f"            if ea.{getter}() != eb.{getter}() {{",
                f'                return Err(format!("{{path}}: {_COMMON_LABELS[field]} mismatch"));',
                "            }",
            ]
        lines.append("        }")
    lines += ["        _ => {}", "    }", "    Ok(())", "}"]
    return "\n".join(lines)


def emit_equality(ir: Ir, overrides: Overrides, cls: ClassIr,
                  available_accessors: set[str] | None = None,
                  arena_names: dict[str, str] | None = None) -> str:
    snake = snake_case(cls.name)
    # body first, then the header: a compare that never touches the other
    # document gets `_other` so rustc's unused-variable lint stays quiet
    body: list[str] = []
    uses_other = False
    for i, base in enumerate(cls.bases):
        accessor = ".base()" if i == 0 else f".{snake_case(base)}()"
        if base == "ARObject":
            # P0's kept compare_ar_object is an associated fn (a, b, path)
            body.append(f"        Self::compare_ar_object(a{accessor}, b{accessor}, path)?;")
        elif (i > 0 and available_accessors is not None
                and snake_case(base) not in available_accessors):
            # pinned template structs may not model a py mixin (the P0
            # PackageableElement embeds no VariationPointCapable); compare
            # only what the template carries
            continue
        else:
            body.append(f"        self.compare_{snake_case(base)}(other, a{accessor}, b{accessor}, path)?;")
            uses_other = True
    for field in cls.fields:
        rf = map_field(field, ir, overrides)
        if available_accessors is not None and f"get_{method_stem(rf.rust_name)}" not in available_accessors:
            continue
        name = method_stem(rf.rust_name)
        label = rf.rust_name.upper()
        if rf.wrapper == "list":
            body += [
                f"        let list_a = a.get_{name}();",
                f"        let list_b = b.get_{name}();",
                "        if list_a.len() != list_b.len() {",
                f'            return Err(format!("{{path}}: {label} length mismatch"));',
                "        }",
                "        for (index, (x, y)) in list_a.iter().zip(list_b.iter()).enumerate() {",
            ]
            if rf.link:
                arena = plural(rf.link) if arena_names is None else arena_names[rf.link]
                body += [
                    f'            let x = self.{arena}.get(*x).ok_or_else(|| format!("{{path}}.{label}[{{index}}]: id not found in own arena"))?;',
                    f'            let y = other.{arena}.get(*y).ok_or_else(|| format!("{{path}}.{label}[{{index}}]: id not found in other arena"))?;',
                    f'            self.compare_{snake_case(rf.link)}(other, x, y, &format!("{{path}}.{label}[{{index}}]"))?;',
                ]
                uses_other = True
            else:
                body += [
                    "            if x != y {",
                    f'                return Err(format!("{{path}}.{label}[{{index}}] mismatch"));',
                    "            }",
                ]
            body.append("        }")
        else:
            body += [
                f"        if a.get_{name}() != b.get_{name}() {{",
                f'            return Err(format!("{{path}}: {label} mismatch"));',
                "        }",
            ]
    body.append("        Ok(())")
    other_param = "other" if uses_other else "_other"
    out = [
        "    pub(crate) fn compare_" + snake + "(",
        "        &self,",
        f"        {other_param}: &Document,",
        f"        a: &{cls.name},",
        f"        b: &{cls.name},",
        "        path: &str,",
        "    ) -> Result<(), String> {",
        *body,
        "    }",
    ]
    return "\n".join(out)


def _remove_fn(text: str, fn_name: str) -> str:
    marker = "    fn " + fn_name + "("
    if marker not in text:
        return text
    start = text.index(marker)
    candidates = [x for x in (text.find("\n    fn ", start + 1), text.find("\n}\n", start)) if x != -1]
    end = min(candidates)
    return text[:start] + text[end + 1:]


def _module_path_of(name: str, ir: Ir, overrides: Overrides, placement: Placement) -> str | None:
    segments = placement.module_of(name)
    if segments:
        return "::".join(segments)
    if name in overrides.alias_module:
        return overrides.alias_module[name]
    cls = ir.get(name)
    if cls is not None:
        from placement import py_module_segments
        segments = py_module_segments(cls.module_py)
        if segments:
            return "::".join(segments)
    return None


def emit_document_root(ir: Ir, overrides: Overrides, placement: Placement) -> str:
    text = templates.DOCUMENT_RS

    concrete = concrete_classes(ir, overrides)
    arena_names = build_arena_names(concrete)

    # 1. arena block regenerated for every concrete class
    arena_lines = [f"    pub(crate) {arena_names[c.name]}: SlotMap<{c.name}Id, {c.name}>," for c in concrete]
    start = text.index(ARENAS_MARKER) + len(ARENAS_MARKER)
    end = text.index(ROOT_FIELDS_MARKER)
    text = text[:start] + "\n" + "\n".join(arena_lines) + "\n\n" + text[end:]

    # 2. drop P0's hand-written per-class resolvers (generated impl replaces them)
    start = text.index(RESOLVERS_MARKER)
    end = text.index(EQUALITY_MARKER)
    text = text[:start] + text[end:]

    # 3. drop the P0 compares that move to generated modules
    for name in REMOVED_COMPARES:
        text = _remove_fn(text, name)

    # 4. import block regenerated: every concrete class' {T, TId} + ElementRef
    groups: dict[str, set[str]] = {"m2::autosar_templates::generic_structure::"
                                   "general_template_classes::ar_object": {"ARObject", "ElementRef"}}
    for c in concrete:
        module = _module_path_of(c.name, ir, overrides, placement)
        if module:
            groups.setdefault(module, set()).update({c.name, c.name + "Id"})
    head_start = text.index("use slotmap::SlotMap;")
    head_end = text.index("/// Root type")
    import_lines = ["use slotmap::SlotMap;", ""]
    for module in sorted(groups):
        names = ", ".join(sorted(groups[module]))
        import_lines.append(f"use crate::{module}::{{{names}}};")
    text = text[:head_start] + "\n".join(import_lines) + "\n\n" + text[head_end:]

    # 5. generated resolvers + element factories, inserted before the tests
    impl: list[str] = ["", "// — generated resolvers (one per concrete class) —", ""]
    for c in concrete:
        resolver = f"get_{snake_case(c.name)}"
        if f"fn {resolver}(" in text:
            # the P0 root keeps a same-name accessor with different arity
            # (get_admin_data() reads the root field, not the arena)
            continue
        impl += [
            f"    pub fn {resolver}(&self, id: {c.name}Id) -> Option<&{c.name}> {{",
            f"        self.{arena_names[c.name]}.get(id)",
            "    }",
            "",
        ]
    impl += ["// — generated element factories (py createXxx directory) —", ""]
    variants = build_element_variants(ir, overrides)
    known = {c.name for c in concrete}
    for name in variants:
        if name == "ARPackage":
            continue  # P0's add_ar_package is kept verbatim
        if name not in known:
            continue
        impl += [
            f"    /// py `create{name}` — allocates in the arena and links the element into the package.",
            f"    pub fn add_{snake_case(name)}(",
            "        &mut self,",
            "        package: Option<ARPackageId>,",
            "        short_name: &str,",
            f"    ) -> {name}Id {{",
            f"        let mut element = {name}::new();",
            "        element.set_short_name(short_name);",
            "        if let Some(package_id) = package {",
            "            element.set_parent(Some(ElementRef::ARPackage(package_id)));",
            "        }",
            f"        let id = self.{arena_names[name]}.insert(element);",
            "        if let Some(package_id) = package {",
            "            if let Some(parent) = self.ar_packages.get_mut(package_id) {",
            f"                parent.push_element(ElementRef::{name}(id));",
            "            }",
            "        }",
            "        id",
            "    }",
            "",
        ]
    insert_at = text.index(TESTS_MARKER)
    text = text[:insert_at] + "impl Document {\n" + "\n".join(impl) + "}\n\n" + text[insert_at:]
    return text
