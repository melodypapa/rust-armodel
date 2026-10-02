# P1 py2rust Converter Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `tools/py2rust/`, a stdlib-only Python 3 tool that converts py-armodel's `models/M2/**` (~1,987 classes) into the committed typed Rust model tree under `src/m2/` (structs, slotmap keys, accessors, enums, cross-module imports, `ElementRef`, tag→constructor registry, `Document` arenas, generated structural equality) — replacing the P0 hand-written model while keeping the P0 parser/writer/CLI/tests compiling and green.

**Architecture:** One generator pipeline: `ast`-based extraction of py classes → module placement from the spec markdown `| Package |` rows → py-type→Rust-kind mapping (with a hand-maintained overrides table for P0-parity quirks) → Rust emission per spec-package module, plus one generated `Document` root. The base chain and `Document` root are **pinned templates** copied verbatim from the current P0 files (including their unit tests), so the generated tree is byte-compatible with what the parser/writer already use. Generated files carry a `@generated` banner.

**Tech Stack:** Python 3 stdlib only (`ast`, `pathlib`, `argparse`, `unittest`) for the tool; existing crate deps only (slotmap, quick-xml, thiserror, clap) on the Rust side.

**Spec:** `docs/superpowers/specs/2026-10-01-rust-armodel-p0-walking-skeleton-design.md` §3 (locked decisions), §11 (P1 definition: converter + generated `ElementRef` + tag→constructor registry).

---

## Prerequisites (before Task 1)

1. **PR #4 (`dep-modernization`) is merged to `main`.** The generated model depends on slotmap. If it is not merged yet, wait — do not branch P1 off the feature branch.
2. **Branch:** `git checkout main && git pull && git checkout -b p1-py2rust-converter`
3. **Reference checkout:** the tool reads a *local* py-armodel clone. Keep it under `target/` so it never pollutes the repo (`target/` is already gitignored):

   ```bash
   mkdir -p target tools/py2rust
   git clone --depth 1 https://github.com/melodypapa/py-armodel.git target/py-armodel
   git -C target/py-armodel rev-parse HEAD > tools/py2rust/PY_ARMODEL_VERSION
   ```

4. **Sanity:** `python3 --version` ≥ 3.9 (uses `ast.unparse`); `cargo build && cargo test` green on the branch point.
5. Work in an isolated worktree if your workflow calls for one (see superpowers:using-git-worktrees).

## Facts about py-armodel the tool relies on (verified 2026-10-02 against `main`)

- Classes: `class AdminData(ARObject):` — single or multiple bases (`class Sdg(ARObject, VariationPointCapable)`, `class Referrable(ARObject, ABC)`).
- Fields are annotated assignments in `__init__`: `self.usedLanguages: Optional[MultiLanguagePlainText] = None`, `self.DocRevisions: List[DocRevision] = []` (note the capital `D` — snake-casing must lowercase the first letter). Mixins (e.g. `VariationPointCapable`) declare fields as **class-level** annotated assignments instead.
- Enumerations extend `AREnum` and declare string-literal class constants (`PRESERVE = "preserve"`); a doc comment precedes each literal.
- Primitive scalars extend `ARType`/`ARLiteral` (`String`, `NameToken`, `RefType`, `DateTime`, `Numerical`, … ~40 of them) — these are **not** emitted; fields typed with them become Rust `String` (P0 precedent: `gid: Optional[NameToken]` → `gid: Option<String>`).
- Constructor quirk: `Referrable.__init__(self, parent, short_name)` — `parent`/`short_name` are constructor params on the whole chain. P0 models them as `Option` fields on `Referrable` with a no-arg `new()`; the extractor hardcodes this skip and the templates carry the P0 shape.
- `ARPackage` (in `ARPackage.py`) carries ~148 `createXxx(short_name)` factories — the "element directory". Each constructs `Xxx(self, short_name)` and `addElement`s it. This set is the source for `ElementRef` variants, `Document::add_xxx` factories, and the tag→constructor registry.
- `AUTOSAR(AbstractAUTOSAR)` is a singleton in py; Rust keeps the P0 `Document` root shape. `AUTOSAR`, `AbstractAUTOSAR`, `AUTOSARDoc`, `FileInfoComment` are skipped via `overrides.skip_emission` (the `Document` template owns this surface).
- Spec markdown `autosar/R23-11/markdown/*.md`: each class has a table whose `| Class | <Name> [(abstract)] |` (or `| Primitive | <Name> |`) row names the class and whose `| Package | M2::A::B::C |` row gives the placement. Tables are split by page breaks; the placement parser tracks the most recent Class/Primitive row per file and binds on the next `| Package |` row.
- py getters may sort (`getARPackages` returns `sorted(..., key=short_name)`). **Rust deliberately returns insertion order** (P0 behavior; round-trip depends on it). Recorded divergence.

## File structure

Tool (all new):

```
tools/py2rust/
├── main.py                 # CLI: --py-armodel PATH --out PATH [--check]; orchestration + coverage report
├── ir.py                   # dataclasses: Ir, ClassIr, FieldIr, EnumLiteral + snake_case
├── extract.py              # ast → Ir (classes, fields, enums, factories, abstract flags)
├── placement.py            # markdown Class/Package rows → class → rust module segments
├── types.py                # py annotation → Rust field kind (String / enum / TId link)
├── overrides.py            # P0-parity quirk table (aliases, skip list, tag overrides)
├── templates.py            # pinned P0 file bodies: base chain, Document root, XmlSpace block
├── emit_model.py           # per-module .rs: imports, enums, structs, key types, accessors, impl Document compare
├── emit_document.py        # ElementRef variants, registry, Document arenas/resolvers/factories, equality dispatch
├── PY_ARMODEL_VERSION      # py-armodel commit the generated code was produced from
└── tests/
    ├── __init__.py
    ├── test_ir.py
    ├── test_extract.py
    ├── test_placement.py
    ├── test_types.py
    ├── test_overrides.py
    └── test_emit.py
```

Generated (replaces the hand-written P0 model files; parser/writer/CLI/tests stay hand-written):

```
src/m2/**                  # one .rs per spec package leaf, banner-marked; mod.rs chains regenerated
src/m2/element_registry.rs # generated tag → constructor registry
```

Unchanged: `src/parser/**`, `src/writer/**`, `src/bin/arxml-dump.rs`, `tests/**`, `Cargo.toml`, `src/lib.rs`.

---

### Task 1: Tool scaffold + IR dataclasses

**Files:**
- Create: `tools/py2rust/ir.py`, `tools/py2rust/tests/test_ir.py`, plus empty `__init__.py` in `tools/py2rust/` and `tools/py2rust/tests/`

- [ ] **Step 1: Write the failing test**

```python
# tools/py2rust/tests/test_ir.py
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
        self.assertEqual(set(ir.classes()), {"Sd", "XmlSpaceEnum"})
        self.assertEqual([c.name for c in ir.enums()], ["XmlSpaceEnum"])


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Run it to verify it fails**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: FAIL (`No module named 'ir'`)

- [ ] **Step 3: Write minimal implementation**

```python
# tools/py2rust/ir.py
"""Intermediate representation shared by extract/emit. Plain dataclasses only."""
from __future__ import annotations

import re
from dataclasses import dataclass, field


def snake_case(name: str) -> str:
    # lowercase first letter, then insert _ between lower/digit and upper
    name = name[0].lower() + name[1:] if name else name
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    return name.replace("__", "_").lower()


@dataclass
class EnumLiteral:
    name: str          # python constant, e.g. PRESERVE
    value: str         # wire value, e.g. "preserve"
    doc: str = ""


@dataclass
class FieldIr:
    name: str                       # python attribute name
    type_expr: str                  # raw annotation, e.g. "Optional[MultiLanguagePlainText]"
    kind: str                       # "optional" | "list"
    inner: str                      # element type name
    doc: str = ""                   # getter docstring (carries the xml.* tags)

    @staticmethod
    def rust_name_for(name: str) -> str:
        return snake_case(name)


@dataclass
class ClassIr:
    name: str
    module_py: str                  # dotted python module
    bases: list[str] = field(default_factory=list)
    doc: str = ""
    is_abstract: bool = False
    is_enum: bool = False
    is_primitive: bool = False      # extends ARType/ARLiteral but not AREnum
    fields: list[FieldIr] = field(default_factory=list)
    enum_literals: list[EnumLiteral] = field(default_factory=list)
    creates: dict[str, str] = field(default_factory=dict)  # factory name -> target class


@dataclass
class Ir:
    _classes: dict[str, ClassIr] = field(default_factory=dict)

    def add(self, cls: ClassIr) -> None:
        self._classes[cls.name] = cls

    def get(self, name: str) -> ClassIr | None:
        return self._classes.get(name)

    def classes(self) -> list[ClassIr]:
        return list(self._classes.values())

    def enums(self) -> list[ClassIr]:
        return [c for c in self._classes.values() if c.is_enum]
```

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (2 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): py2rust scaffold with IR dataclasses"
```

---

### Task 2: AST extractor — class skeletons and enums

**Files:**
- Create: `tools/py2rust/extract.py`
- Create: `tools/py2rust/tests/test_extract.py`

- [ ] **Step 1: Write the failing test**

```python
# tools/py2rust/tests/test_extract.py
import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from extract import extract_source

SAMPLE = '''
from typing import List, Optional
from x import ARObject, AREnum, ARLiteral


class Modification(ARObject):
    """Records what has changed."""

    def __init__(self):
        super().__init__()
        self.change: Optional[MultiLanguageOverviewParagraph] = None


class Referrable(ARObject, ABC):
    """Can be referred to by its identifier."""

    def __init__(self, parent: ARObject, short_name: str):
        if type(self) is Referrable:
            raise TypeError("Referrable is an abstract class.")
        ARObject.__init__(self)
        self.parent: ARObject = parent
        self.short_name: str = short_name


class XmlSpaceEnum(AREnum):
    DEFAULT = "default"
    PRESERVE = "preserve"

    def __init__(self):
        super().__init__((XmlSpaceEnum.DEFAULT, XmlSpaceEnum.PRESERVE))


class NameToken(ARLiteral):
    pass
'''


class TestExtractSkeleton(unittest.TestCase):
    def setUp(self):
        self.ir = extract_source(SAMPLE, "armodel.models.M2.test")

    def test_classes_registered_with_bases(self):
        self.assertEqual(self.ir.get("Modification").bases, ["ARObject"])
        self.assertEqual(self.ir.get("Referrable").bases, ["ARObject"])  # ABC dropped

    def test_abstract_flag_from_typeerror_guard_and_abc_base(self):
        self.assertFalse(self.ir.get("Modification").is_abstract)
        self.assertTrue(self.ir.get("Referrable").is_abstract)

    def test_enum_and_primitive_flags(self):
        self.assertTrue(self.ir.get("XmlSpaceEnum").is_enum)
        self.assertFalse(self.ir.get("XmlSpaceEnum").is_abstract)
        self.assertTrue(self.ir.get("NameToken").is_primitive)

    def test_enum_literals_read_wire_values_with_doc(self):
        literals = {l.name: (l.value, l.doc) for l in self.ir.get("XmlSpaceEnum").enum_literals}
        self.assertEqual(literals["PRESERVE"][0], "preserve")


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Run it to verify it fails**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: FAIL (`cannot import name 'extract_source'`)

- [ ] **Step 3: Write minimal implementation**

```python
# tools/py2rust/extract.py
"""ast-based extraction of py-armodel model classes into the IR."""
from __future__ import annotations

import ast
import pathlib

from ir import ClassIr, EnumLiteral, FieldIr, Ir

ENUM_BASES = {"AREnum"}
PRIMITIVE_BASES = {"ARType", "ARLiteral"}


def _class_doc(node: ast.ClassDef) -> str:
    return ast.get_docstring(node) or ""


def _is_abstract(node: ast.ClassDef) -> bool:
    base_names = [b.id for b in node.bases if isinstance(b, ast.Name)]
    if "ABC" in base_names:
        return True
    for stmt in ast.walk(node):
        if isinstance(stmt, ast.Raise) and "abstract class" in ast.unparse(stmt):
            return True
    return False


def _preceding_doc(source_lines: list[str], lineno: int) -> str:
    """Doc = consecutive '#' comment lines immediately above the statement."""
    doc_lines: list[str] = []
    start = lineno - 2  # 1-based lineno -> index of previous line
    while start >= 0:
        line = source_lines[start].strip()
        if line.startswith("#"):
            doc_lines.insert(0, line.lstrip("# ").strip())
            start -= 1
            continue
        break
    return " ".join(doc_lines)


def _enum_literals(node: ast.ClassDef, source_lines: list[str]) -> list[EnumLiteral]:
    literals = []
    for stmt in node.body:
        if not (isinstance(stmt, ast.Assign) and len(stmt.targets) == 1):
            continue
        target = stmt.targets[0]
        if not (isinstance(target, ast.Name) and isinstance(stmt.value, ast.Constant)
                and isinstance(stmt.value.value, str)):
            continue
        literals.append(EnumLiteral(name=target.id, value=stmt.value.value,
                                    doc=_preceding_doc(source_lines, stmt.lineno)))
    return literals


def extract_source(source: str, module_py: str) -> Ir:
    ir = Ir()
    tree = ast.parse(source)
    source_lines = source.splitlines()
    for node in tree.body:
        if not isinstance(node, ast.ClassDef):
            continue
        bases = [b.id for b in node.bases if isinstance(b, ast.Name) and b.id not in ("ABC", "object")]
        is_enum = any(b in ENUM_BASES for b in bases)
        is_primitive = (not is_enum) and any(b in PRIMITIVE_BASES for b in bases)
        cls = ClassIr(
            name=node.name,
            module_py=module_py,
            bases=bases,
            doc=_class_doc(node),
            is_abstract=_is_abstract(node),
            is_enum=is_enum,
            is_primitive=is_primitive,
        )
        if is_enum:
            cls.enum_literals = _enum_literals(node, source_lines)
        ir.add(cls)
    return ir


def extract_repo(root: str) -> Ir:
    """Extract every models/M2/**/*.py under a py-armodel checkout root."""
    ir = Ir()
    base = pathlib.Path(root) / "src/armodel/models/M2"
    for path in sorted(base.rglob("*.py")):
        if path.name.startswith("test_"):
            continue
        rel = path.relative_to(base.parent.parent)          # src/...
        module_py = ".".join(rel.with_suffix("").parts)
        for cls in extract_source(path.read_text(), module_py).classes():
            ir.add(cls)
    return ir
```

(Field extraction is Task 3; `FieldIr` is imported for later use — if your linter complains, drop the import here and re-add in Task 3.)

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (6 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): extract class skeletons, abstract/enum/primitive flags"
```

---

### Task 3: AST extractor — fields and create-factories

**Files:**
- Modify: `tools/py2rust/extract.py`
- Modify: `tools/py2rust/tests/test_extract.py`

- [ ] **Step 1: Append the failing tests to `test_extract.py`**

```python
FIELD_SAMPLE = '''
from typing import List, Optional
from x import ARObject


class DocRevision(ARObject):
    def __init__(self):
        super().__init__()
        # This specifies the date and time. Tags: xml.sequenceOffset=80
        self.date: Optional[DateTime] = None
        self.issuedBy: Optional[String] = None
        self.modifications: List[Modification] = []


class AdminData(ARObject):
    def __init__(self):
        super().__init__()
        self.DocRevisions: List[DocRevision] = []
        self.language: Optional[LEnum] = None
        self.sdgs: List[Sdg] = []
        self.usedLanguages: Optional[MultiLanguagePlainText] = None

    def getUsedLanguages(self) -> Optional[MultiLanguagePlainText]:
        """This property specifies the languages. Tags: xml.sequenceOffset=30"""
        return self.usedLanguages


class SdgCaption(ARObject):
    def __init__(self, parent, short_name):
        super().__init__(parent, short_name)
        self.desc: Optional[MultiLanguageOverviewParagraph] = None


class VariationPointCapable(ABC):
    variationPoint: Optional[VariationPoint] = None

    def getVariationPoint(self) -> Optional[VariationPoint]:
        return self.variationPoint


class ARPackage(ARObject):
    def __init__(self):
        super().__init__()
        self.arPackages: List[ARPackage] = []

    def createARPackage(self, short_name: str) -> ARPackage:
        ar_package = ARPackage(self, short_name)
        self.arPackages.append(ar_package)
        return ar_package

    def createApplicationSwComponentType(self, short_name: str) -> ApplicationSwComponentType:
        sw_component = ApplicationSwComponentType(self, short_name)
        self.addElement(sw_component)
        return sw_component
'''


class TestExtractFields(unittest.TestCase):
    def setUp(self):
        self.ir = extract_source(FIELD_SAMPLE, "armodel.models.M2.test")

    def test_optional_and_list_fields(self):
        doc_revision = self.ir.get("DocRevision")
        by_name = {f.name: f for f in doc_revision.fields}
        self.assertEqual(by_name["date"].kind, "optional")
        self.assertEqual(by_name["date"].inner, "DateTime")
        self.assertEqual(by_name["modifications"].kind, "list")
        self.assertEqual(by_name["modifications"].inner, "Modification")
        self.assertIn("xml.sequenceOffset=80", by_name["date"].doc)

    def test_getter_docstring_wins_over_comment(self):
        admin = {f.name: f for f in self.ir.get("AdminData").fields}
        self.assertIn("xml.sequenceOffset=30", admin["usedLanguages"].doc)

    def test_constructor_params_parent_short_name_are_not_fields(self):
        self.assertEqual([f.name for f in self.ir.get("SdgCaption").fields], ["desc"])

    def test_class_level_mixin_fields(self):
        mixin = self.ir.get("VariationPointCapable")
        self.assertEqual([f.name for f in mixin.fields], ["variationPoint"])
        self.assertEqual(mixin.fields[0].kind, "optional")

    def test_create_factories_collected_by_name(self):
        creates = self.ir.get("ARPackage").creates
        self.assertEqual(creates["createARPackage"], "ARPackage")
        self.assertEqual(creates["createApplicationSwComponentType"], "ApplicationSwComponentType")
```

- [ ] **Step 2: Run to verify failure**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: new tests FAIL (fields/creates empty)

- [ ] **Step 3: Implement — add to `extract.py` and hook into `extract_source`**

```python
def _annotation_parts(annotation) -> tuple[str, str] | None:
    """Return (kind, inner) for Optional[T] / List[T]; None otherwise."""
    if not (isinstance(annotation, ast.Subscript)
            and isinstance(annotation.value, ast.Name)
            and annotation.value.id in ("Optional", "List")):
        return None
    kind = "optional" if annotation.value.id == "Optional" else "list"
    if isinstance(annotation.slice, ast.Name):
        return kind, annotation.slice.id
    return None


def _getter_doc(node: ast.ClassDef, field_name: str) -> str:
    getter = "get" + field_name[0].upper() + field_name[1:]
    for stmt in node.body:
        if isinstance(stmt, ast.FunctionDef) and stmt.name == getter:
            return ast.get_docstring(stmt) or ""
    return ""


def _extract_fields(node: ast.ClassDef, source_lines: list[str]) -> list[FieldIr]:
    fields: list[FieldIr] = []

    def add(annotation, name: str, declared_at: int) -> None:
        if name in ("parent", "short_name"):  # constructor-param quirk: P0 template owns these
            return
        parts = _annotation_parts(annotation)
        if parts is None:
            return
        kind, inner = parts
        fields.append(FieldIr(
            name=name,
            type_expr=f"{kind.capitalize()}[{inner}]",
            kind=kind,
            inner=inner,
            doc=_getter_doc(node, name) or _preceding_doc(source_lines, declared_at),
        ))

    for stmt in node.body:  # class-level annotated assignments (mixins)
        if isinstance(stmt, ast.AnnAssign) and isinstance(stmt.target, ast.Name):
            add(stmt.annotation, stmt.target.id, stmt.lineno)
    for stmt in node.body:  # __init__ annotated assignments
        if not (isinstance(stmt, ast.FunctionDef) and stmt.name == "__init__"):
            continue
        for sub in ast.walk(stmt):
            if (isinstance(sub, ast.AnnAssign) and isinstance(sub.target, ast.Attribute)
                    and isinstance(sub.target.value, ast.Name) and sub.target.value.id == "self"):
                add(sub.annotation, sub.target.attr, sub.lineno)
    return fields


def _extract_creates(node: ast.ClassDef) -> dict[str, str]:
    creates: dict[str, str] = {}
    for stmt in node.body:
        if not (isinstance(stmt, ast.FunctionDef) and stmt.name.startswith("create")):
            continue
        for sub in ast.walk(stmt):
            if (isinstance(sub, ast.Call) and isinstance(sub.func, ast.Name)
                    and sub.func.id[:1].isupper() and sub.func.id not in ("TypeError",)):
                creates[stmt.name] = sub.func.id  # keep the py name; snake-case at emission
                break
    return creates
```

Hook both into `extract_source` (right before `ir.add(cls)`):

```python
        if not cls.is_enum and not cls.is_primitive:
            cls.fields = _extract_fields(node, source_lines)
            cls.creates = _extract_creates(node)
```

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (11 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): extract fields (init/class-level) and create-factories"
```

---

### Task 4: Module placement from spec markdown

**Files:**
- Create: `tools/py2rust/placement.py`
- Create: `tools/py2rust/tests/test_placement.py`

- [ ] **Step 1: Write the failing test**

```python
# tools/py2rust/tests/test_placement.py
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
```

- [ ] **Step 2: Run to verify failure**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: FAIL (`No module named 'placement'`)

- [ ] **Step 3: Implement**

```python
# tools/py2rust/placement.py
"""Map spec classes to rust module paths via the markdown | Class | / | Package | rows."""
from __future__ import annotations

import re
import pathlib

from ir import snake_case

CLASS_ROW = re.compile(r"^\|\s*Class\s*\|\s*([A-Za-z][A-Za-z0-9_]*)(\s*\(abstract\))?\s*\|")
PRIMITIVE_ROW = re.compile(r"^\|\s*Primitive\s*\|\s*([A-Za-z][A-Za-z0-9_]*)\s*\|")
PACKAGE_ROW = re.compile(r"^\|\s*Package\s*\|\s*(M2::[A-Za-z0-9:]+)\s*\|")


def package_to_segments(package: str) -> list[str]:
    return [snake_case(part) for part in package.split("::")]


class Placement:
    def __init__(self, class_to_package: dict[str, str]):
        self._by_class = class_to_package

    def module_of(self, class_name: str) -> list[str] | None:
        package = self._by_class.get(class_name)
        return package_to_segments(package) if package else None


def parse_markdown(files: dict[str, str]) -> Placement:
    """files: markdown file name -> content. Tables split across page breaks are
    handled by tracking the most recent Class/Primitive row per file."""
    binding: dict[str, str] = {}
    for content in files.values():
        current: str | None = None
        for line in content.splitlines():
            match = CLASS_ROW.match(line) or PRIMITIVE_ROW.match(line)
            if match:
                current = match.group(1)
                continue
            match = PACKAGE_ROW.match(line)
            if match and current:
                binding[current] = match.group(1)
    return Placement(binding)


def parse_repo(markdown_root: str) -> Placement:
    files = {p.name: p.read_text(errors="replace")
             for p in sorted(pathlib.Path(markdown_root).glob("*.md"))}
    return parse_markdown(files)
```

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (15 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): placement from spec markdown Class/Package rows"
```

---

### Task 5: Type mapping + overrides table

**Files:**
- Create: `tools/py2rust/types.py`, `tools/py2rust/overrides.py`
- Create: `tools/py2rust/tests/test_types.py`, `tools/py2rust/tests/test_overrides.py`

- [ ] **Step 1: Write the failing tests**

```python
# tools/py2rust/tests/test_types.py
import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from ir import ClassIr, FieldIr, Ir
from overrides import Overrides
from types import RustField, map_field


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

    def test_unknown_inner_is_reported_not_crashed(self):
        field = FieldIr(name="weird", type_expr="Optional[Mystery]", kind="optional", inner="Mystery")
        with self.assertRaisesRegex(KeyError, "Mystery"):
            map_field(field, self.ir, self.overrides)
```

```python
# tools/py2rust/tests/test_overrides.py
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
```

- [ ] **Step 2: Run to verify failure**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: FAIL (`No module named 'overrides'`; note: our `types.py` shadows stdlib `types` only inside the tool's `sys.path` — that is fine for a self-contained tool)

- [ ] **Step 3: Implement**

```python
# tools/py2rust/overrides.py
"""Hand-maintained P0-parity quirks. Everything not listed here converts mechanically."""
from __future__ import annotations

import re


def kebab_tag(class_name: str) -> str:
    return re.sub(r"(?<=[a-z0-9])([A-Z])|(?<=[A-Z])([A-Z][a-z])", r"-\1\2", class_name).upper()


class Overrides:
    def __init__(self, *, type_alias, skip_emission, tag_overrides):
        self.type_alias_map = type_alias
        self.skip_emission = skip_emission
        self.tag_overrides = tag_overrides

    @staticmethod
    def default() -> "Overrides":
        return Overrides(
            # generated enum name -> existing hand-written Rust type (not regenerated)
            type_alias={"XmlSpaceEnum": "XmlSpace"},
            # not emitted as Rust: aliased, pinned templates, or primitive meta-bases
            skip_emission={
                "XmlSpaceEnum",                                   # aliased to hand-written XmlSpace
                "ARType", "ARLiteral", "AREnum",                  # primitive meta-bases
                # pinned templates (base chain + root), copied verbatim in templates.py:
                "ARObject", "Referrable", "MultilanguageReferrable", "Identifiable",
                "CollectableElement", "PackageableElement", "ARElement", "ARPackage",
                "ReferenceBase", "ShortNameFragment", "Describable",
                "AUTOSAR", "AbstractAUTOSAR", "AUTOSARDoc", "FileInfoComment",
                "SingleLanguageReferrable", "MultiLanguageParagraph", "MultilanguageLongName",
                "MultiLanguageVerbatim",
            },
            # kebab rule misfires on digit/acronym runs; fixed tags verified against py parser in P2
            tag_overrides={},
        )

    def type_alias(self, name: str) -> str | None:
        return self.type_alias_map.get(name)

    def emits(self, name: str) -> bool:
        return name not in self.skip_emission

    def tag_for(self, class_name: str) -> str:
        return self.tag_overrides.get(class_name, kebab_tag(class_name))
```

```python
# tools/py2rust/types.py
"""Map py field annotations to Rust field kinds."""
from __future__ import annotations

from dataclasses import dataclass

from ir import FieldIr, Ir
from overrides import Overrides

# py-armodel scalar primitives (PrimitiveTypes.py, non-enum) -> Rust String.
# Values round-trip verbatim, matching P0 (P0 design §5: NameToken/VerbatimStringPlain → String).
# AREnum subclasses (all *Enum names) are NOT here — they become generated Rust enums.
PRIMITIVES = {
    "String", "UriString", "AlignmentType", "SectionInitializationPolicyType", "CseCodeType",
    "DisplayFormatString", "NativeDeclarationString", "BaseTypeEncodingString",
    "PrimitiveIdentifier", "PositiveInteger", "Boolean", "NameToken",
    "PositiveUnlimitedInteger", "Integer", "UnlimitedInteger", "Identifier", "CIdentifier",
    "RevisionLabelString", "Ref", "RefType", "TRefType", "DiagRequirementIdString",
    "Ip4AddressString", "Ip6AddressString", "MacAddressString", "CategoryString",
    "AnyServiceInstanceId", "AnyVersionString", "DateTime", "VerbatimString",
    "VerbatimStringPlain", "RegularExpression", "SymbolString", "McdIdentifier",
    "MimeTypeString", "NameTokens", "ViewTokens", "Numerical", "Float", "TimeValue",
}


@dataclass
class RustField:
    rust_name: str
    rust_type: str      # element type as written in Rust: String / <Enum> / <T>Id
    wrapper: str        # "optional" | "list"
    link: str | None    # None for owned scalars/enums; "<T>" for arena links


def map_field(field: FieldIr, ir: Ir, overrides: Overrides) -> RustField:
    inner = field.inner
    alias = overrides.type_alias(inner)
    if alias:
        return RustField(FieldIr.rust_name_for(field.name), alias, field.wrapper, link=None)
    if inner in PRIMITIVES:
        return RustField(FieldIr.rust_name_for(field.name), "String", field.wrapper, link=None)
    cls = ir.get(inner)
    if cls is None:
        raise KeyError(f"unknown field type {inner!r} (field {field.name}); add to PRIMITIVES or overrides")
    if cls.is_enum:
        return RustField(FieldIr.rust_name_for(field.name), inner, field.wrapper, link=None)
    return RustField(FieldIr.rust_name_for(field.name), f"{inner}Id", field.wrapper, link=inner)
```

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (23 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): py->Rust type mapping and P0-parity overrides table"
```

---

### Task 6: Model emitter — imports, enums, structs, keys, accessors

**Files:**
- Create: `tools/py2rust/emit_model.py`
- Create: `tools/py2rust/tests/test_emit.py`

- [ ] **Step 1: Write the failing tests (golden emission for a P0-fixture subset)**

```python
# tools/py2rust/tests/test_emit.py
import sys, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from emit_model import emit_module
from extract import extract_source
from ir import ClassIr, Ir
from overrides import Overrides
from placement import Placement

FIXTURE = '''
from x import ARObject, AREnum, NameToken, VerbatimStringPlain
from typing import List, Optional


class Sd(ARObject):
    """This class represents a primitive element in a special data group."""

    def __init__(self):
        super().__init__()
        self.gid: Optional[NameToken] = None
        self.value: Optional[VerbatimStringPlain] = None
        self.xmlSpace: Optional[XmlSpaceEnum] = None


class Sdg(ARObject, VariationPointCapable):
    def __init__(self):
        super().__init__()
        self.gid: Optional[NameToken] = None
        self.sdgCaption: Optional[SdgCaption] = None
        self.sdgContentsType: Optional[SdgContents] = None


class SdgContents(ARObject):
    def __init__(self):
        super().__init__()
        self.sd: List[Sd] = []
        self.sdf: List[Sdf] = []
        self.sdg: List[Sdg] = []


class BindingTimeEnum(AREnum):
    CODE_GENERATION_TIME = "codeGenerationTime"
    LINK_TIME = "linkTime"
'''

# placement knows the fixture classes; Sdg/SdgCaption/Sdf live in another module
PLACEMENT = Placement({
    "Sd": "M2::Test::SpecialData", "Sdg": "M2::Test::SpecialData",
    "SdgContents": "M2::Test::SpecialData", "SdgCaption": "M2::Other",
    "Sdf": "M2::Other", "ARObject": "M2::Base", "VariationPointCapable": "M2::Base",
})


class TestEmitModule(unittest.TestCase):
    def setUp(self):
        self.ir = extract_source(FIXTURE, "m")
        self.overrides = Overrides.default()

    def test_enum_only_module_has_no_key_types(self):
        out = emit_module(self.ir, self.overrides, PLACEMENT,
                          [self.ir.get("BindingTimeEnum")], ["m2", "test"])
        self.assertNotIn("new_key_type!", out)
        self.assertIn("#[derive(Debug, Clone, Copy, PartialEq, Eq)]", out)
        self.assertIn("pub enum BindingTimeEnum {", out)
        self.assertIn('CodeGenerationTime => "codeGenerationTime"', out)

    def test_sd_struct_golden(self):
        out = emit_module(self.ir, self.overrides, PLACEMENT,
                          [self.ir.get("Sd")], ["m2", "test"])
        self.assertIn("// @generated by tools/py2rust", out)
        self.assertIn("pub struct Sd {", out)
        self.assertIn("base: ARObject,", out)
        self.assertIn("gid: Option<String>,", out)
        self.assertIn("xml_space: Option<XmlSpace>,", out)
        self.assertIn("pub fn get_gid(&self) -> Option<&str>", out)
        self.assertIn("pub fn set_gid(&mut self, value: impl Into<String>) -> &mut Self", out)
        self.assertIn("pub fn get_xml_space(&self) -> Option<XmlSpace>", out)
        self.assertIn("use crate::m2::base::ar_object::ARObject;", out)
        self.assertIn("use crate::Document;", out)  # equality impl needs it

    def test_link_fields_key_types_and_push_accessors(self):
        out = emit_module(self.ir, self.overrides, PLACEMENT,
                          [self.ir.get("Sdg"), self.ir.get("SdgContents")], ["m2", "test"])
        self.assertIn("pub struct SdgId;", out)
        self.assertIn("pub struct SdgContentsId;", out)
        self.assertIn("sdg_caption: Option<SdgCaptionId>,", out)
        self.assertIn("pub fn get_sd(&self) -> &[SdId]", out)
        self.assertIn("pub fn push_sd(&mut self, value: SdId)", out)
        self.assertIn("use crate::m2::other::SdgCaptionId;", out)


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Run to verify failure**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: FAIL (`No module named 'emit_model'`)

- [ ] **Step 3: Implement `emit_model.py`**

```python
# tools/py2rust/emit_model.py
"""Emit one rust module file: imports, enums, structs, key types, accessors, compare fn."""
from __future__ import annotations

from ir import ClassIr, FieldIr, Ir, snake_case
from overrides import Overrides
from placement import Placement
from types import RustField, map_field

BANNER = "// @generated by tools/py2rust from py-armodel models/M2 (see PY_ARMODEL_VERSION). Do not edit.\n"


def enum_variant_name(literal: str) -> str:
    return "".join(part.capitalize() for part in literal.lower().split("_"))


def _emit_enum(cls: ClassIr) -> str:
    lines = [
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub enum {cls.name} {{",
    ]
    lines += [f"    {enum_variant_name(l.name)}," for l in cls.enum_literals]
    lines += ["}", "", f"impl {cls.name} {{", "    pub fn as_str(&self) -> &'static str {", "        match self {"]
    lines += [f'            {cls.name}::{enum_variant_name(l.name)} => "{l.value}",' for l in cls.enum_literals]
    lines += ["        }", "    }", "}", "",
              f"impl TryFrom<&str> for {cls.name} {{", "    type Error = ();", "",
              "    fn try_from(value: &str) -> Result<Self, Self::Error> {", "        match value {"]
    lines += [f'            "{l.value}" => Ok({cls.name}::{enum_variant_name(l.name)}),' for l in cls.enum_literals]
    lines += ["            _ => Err(()),", "        }", "    }", "}"]
    return "\n".join(lines)


def _emit_struct_field(rf: RustField) -> str:
    if rf.wrapper == "list":
        return f"    {rf.rust_name}: Vec<{rf.rust_type}>,"
    return f"    {rf.rust_name}: Option<{rf.rust_type}>,"


def _emit_accessors(rf: RustField) -> list[str]:
    name, t = rf.rust_name, rf.rust_type
    if rf.wrapper == "list":
        return [
            f"    pub fn get_{name}(&self) -> &[{t}] {{",
            f"        &self.{name}",
            "    }",
            "",
            f"    pub fn push_{name}(&mut self, value: {t}) {{",
            f"        self.{name}.push(value);",
            "    }",
            "",
        ]
    if t == "String":
        return [
            f"    pub fn get_{name}(&self) -> Option<&str> {{",
            f"        self.{name}.as_deref()",
            "    }",
            "",
            f"    pub fn set_{name}(&mut self, value: impl Into<String>) -> &mut Self {{",
            f"        self.{name} = Some(value.into());",
            "        self",
            "    }",
            "",
        ]
    if rf.link:
        return [
            f"    pub fn get_{name}(&self) -> Option<{t}> {{",
            f"        self.{name}",
            "    }",
            "",
            f"    pub fn set_{name}(&mut self, value: {t}) -> &mut Self {{",
            f"        self.{name} = Some(value);",
            "        self",
            "    }",
            "",
        ]
    # Copy enum
    return [
        f"    pub fn get_{name}(&self) -> Option<{t}> {{",
        f"        self.{name}",
        "    }",
        "",
        f"    pub fn set_{name}(&mut self, value: {t}) -> &mut Self {{",
        f"        self.{name} = Some(value);",
        "        self",
        "    }",
        "",
    ]


def _base_field(base: str, first: bool) -> str:
    return f"    base: {base}," if first else f"    {snake_case(base)}: {base},"


def _base_accessor(base: str, first: bool) -> list[str]:
    if first:
        return [f"    pub fn base(&self) -> &{base} {{", "        &self.base", "    }", "",
                f"    pub fn base_mut(&mut self) -> &mut {base} {{", "        &mut self.base", "    }", ""]
    name = snake_case(base)
    return [f"    pub fn {name}(&self) -> &{base} {{", f"        &self.{name}", "    }", ""]


def _emit_struct(cls: ClassIr, ir: Ir, overrides: Overrides) -> list[str]:
    lines = [f"/// spec class `{cls.name}`" + (" (abstract)" if cls.is_abstract else "")]
    if cls.doc:
        lines += [f"/// {line}".rstrip() for line in cls.doc.splitlines()[:3] if line.strip()]
    lines += ["#[derive(Debug, Default)]", f"pub struct {cls.name} {{"]
    rust_fields = []
    for i, base in enumerate(cls.bases):
        lines.append(_base_field(base, i == 0))
    for f in cls.fields:
        rf = map_field(f, ir, overrides)
        rust_fields.append(rf)
        lines.append(_emit_struct_field(rf))
    lines += ["}", "", f"impl {cls.name} {{", "    pub fn new() -> Self {", "        Self::default()", "    }", ""]
    for i, base in enumerate(cls.bases):
        lines.extend(_base_accessor(base, i == 0))
    for rf in rust_fields:
        lines.extend(_emit_accessors(rf))
    lines += ["}"]
    return lines


def _rust_module_of(type_name: str, placement: Placement) -> str | None:
    segments = placement.module_of(type_name)
    return "::".join(segments) if segments else None


def collect_imports(ir: Ir, overrides: Overrides, placement: Placement,
                    classes: list[ClassIr]) -> list[str]:
    """use-lines for every referenced type not defined in this module + Document."""
    own = {c.name for c in classes}
    referenced: set[str] = set()
    for cls in classes:
        if overrides.emits(cls.name) is False:
            continue
        referenced.update(b for b in cls.bases if b not in own)
        for f in cls.fields:
            alias = overrides.type_alias(f.inner)
            if alias:
                referenced.add(alias)
                continue
            if f.inner in own:
                continue
            target = ir.get(f.inner)
            if target and target.is_enum:
                referenced.add(f.inner)      # generated enum, lives in its own module
            elif not target:
                referenced.add(f.inner)      # primitive alias types live somewhere; resolved below
    lines = []
    for name in sorted(referenced):
        module = _rust_module_of(name, placement)
        if module:
            lines.append(f"use crate::{module}::{name};")
        elif name in ("String",):
            continue
        # hand-written pinned types (XmlSpace etc.) resolve via their pinned module:
        # the templates step injects `use crate::m2::…language_data_model::XmlSpace;` when needed.
    lines.append("use crate::Document;")  # the generated compare fn needs it
    return sorted(set(lines))


def emit_module(ir: Ir, overrides: Overrides, placement: Placement,
                classes: list[ClassIr], module_segments: list[str]) -> str:
    classes = sorted(classes, key=lambda c: c.name)
    out = [BANNER, f"//! generated from spec package: {'::'.join(module_segments)}", ""]
    concrete = [c for c in classes if not c.is_abstract and not c.is_enum and overrides.emits(c.name)]
    if concrete:
        out += ["use slotmap::new_key_type;", ""]
        out += ["new_key_type! {"]
        out += [f"    pub struct {c.name}Id;" for c in concrete]
        out += ["}", ""]
    out += [line for line in collect_imports(ir, overrides, placement, classes) if line]
    out.append("")
    for cls in classes:
        if cls.is_enum and overrides.emits(cls.name):
            out.append(_emit_enum(cls))
            out.append("")
    for cls in classes:
        if overrides.emits(cls.name) and not cls.is_enum:
            out.extend(_emit_struct(cls, ir, overrides))
            out.append("")
    return "\n".join(out)
```

Implementation notes:
- The Compare-fn emission (`impl Document` block per class) is added in Task 8's wiring; `collect_imports` already imports `Document` for it.
- Hand-written pinned types referenced through aliases (only `XmlSpace` today): the module that hosts the aliased type is `language_data_model`, which is produced from `templates.py` (Task 7) and already contains/imports it. When `map_field` returns an aliased type in *other* modules, `collect_imports` can't resolve it from placement — resolve with a tiny hardcoded map in `overrides.py`: `alias_module = {"XmlSpace": "m2::msr::documentation::text_model::language_data_model"}` and emit that use-line. Add this to `Overrides.default()` and use it in `collect_imports` (replace the comment branch above with a lookup there).

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (26 tests). The emitted `Sd` must be accessor-compatible with the current hand-written `Sd` (`get_gid -> Option<&str>`, `set_gid(impl Into<String>) -> &mut Self`, `get_xml_space -> Option<XmlSpace>`).

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): rust model emitter (imports, enums, structs, keys, accessors)"
```

---

### Task 7: Pinned templates, ElementRef, registry, generated equality

**Files:**
- Create: `tools/py2rust/templates.py`
- Create: `tools/py2rust/emit_document.py`
- Modify: `tools/py2rust/tests/test_emit.py`

- [ ] **Step 1: Copy the P0 bodies into `templates.py` — verbatim, whole files**

The base chain and `Document` root must stay byte-compatible with what the P0 parser/writer use, **including their `#[cfg(test)]` unit tests** (they are the regression net). Copy the *entire current content* of these five files into `templates.py` as named string constants (adjusting only: add the `@generated` banner as the first line, and in `AR_OBJECT_RS` extend the `ElementRef` enum with a marker comment where variants will be appended):

| Constant | Source file (verbatim) |
|---|---|
| `AR_OBJECT_RS` | `src/m2/autosar_templates/generic_structure/general_template_classes/ar_object.rs` |
| `IDENTIFIABLE_RS` | `src/m2/autosar_templates/generic_structure/general_template_classes/identifiable.rs` |
| `ELEMENT_COLLECTION_RS` | `src/m2/autosar_templates/generic_structure/general_template_classes/element_collection.rs` |
| `AR_PACKAGE_RS` | `src/m2/autosar_templates/generic_structure/general_template_classes/ar_package.rs` |
| `DOCUMENT_RS` | `src/m2/autosar_templates/autosar_top_level_structure.rs` |
| `XML_SPACE_BLOCK` | the `XmlSpace` enum + `TryFrom` + `Display` impls from `src/m2/msr/documentation/text_model/language_data_model.rs` (only that block; `LPlainText` is generated normally) |

```python
# tools/py2rust/templates.py
"""Pinned P0 bodies (base chain, Document root, XmlSpace) — byte-compat with parser/writer.

Source of truth: the P0 files listed per constant. Copy them verbatim here; the only
edits are the @generated banner line and the ElementRef variant marker in AR_OBJECT_RS.
"""
AR_OBJECT_RS = """\
// @generated — pinned P0 base (see tools/py2rust/templates.py). Do not edit.
<full current content of ar_object.rs, with the enum body ending in:>
    ARPackage(ARPackageId),
    // %%ELEMENT_VARIANTS%% — appended by tools/py2rust
"""
# IDENTIFIABLE_RS / ELEMENT_COLLECTION_RS / AR_PACKAGE_RS / DOCUMENT_RS / XML_SPACE_BLOCK:
# full current file contents, verbatim, banner line prepended.
```

- [ ] **Step 2: Append the failing tests to `test_emit.py`**

```python
from emit_document import build_element_variants, emit_registry, emit_equality


class TestEmitDocument(unittest.TestCase):
    def setUp(self):
        ir = extract_source(FIXTURE, "m")
        ir.get("ARPackage").creates = {
            "createARPackage": "ARPackage",
            "createApplicationSwComponentType": "ApplicationSwComponentType",
        }
        ir.add(ClassIr(name="ApplicationSwComponentType", module_py="m", bases=["ARObject"]))
        self.ir = ir

    def test_element_variants_from_create_directory(self):
        variants = build_element_variants(self.ir)
        self.assertEqual(variants, ["ARPackage", "ApplicationSwComponentType"])

    def test_registry_maps_kebab_tags(self):
        code = emit_registry(self.ir, Overrides.default(), ["ARPackage", "ApplicationSwComponentType"])
        self.assertIn('"APPLICATION-SW-COMPONENT-TYPE" =>', code)
        self.assertIn("ElementRef::ApplicationSwComponentType(d.add_application_sw_component_type(p, n))", code)
        self.assertIn('"AR-PACKAGE" =>', code)

    def test_equality_emits_scalar_and_link_compare(self):
        code = emit_equality(self.ir, self.ir.get("SdgContents"))
        self.assertIn("fn compare_sdg_contents(", code)
        self.assertIn("a.get_sd().len() != b.get_sd().len()", code)
        self.assertIn('format!("{path}.SD[{index}]: id not found in own arena"', code)
        self.assertIn("self.compare_sd(other, x, y,", code)
```

- [ ] **Step 3: Run to verify failure, then implement `emit_document.py`**

```python
# tools/py2rust/emit_document.py
"""ElementRef variants, tag→constructor registry, generated equality, Document arenas."""
from __future__ import annotations

from ir import ClassIr, Ir, snake_case
from overrides import Overrides
from types import map_field


def build_element_variants(ir: Ir) -> list[str]:
    """ARPackage first, then every class ARPackage's createXxx directory constructs."""
    variants = ["ARPackage"]
    for _factory, target in sorted(ir.get("ARPackage").creates.items()):
        cls = ir.get(target)
        if target not in variants and cls and not cls.is_abstract:
            variants.append(target)
    return variants


def emit_registry(ir: Ir, overrides: Overrides, variants: list[str]) -> str:
    lines = [
        "// @generated by tools/py2rust — tag → constructor registry. Do not edit.",
        "use crate::Document;",
        "use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_object::ElementRef;",
        "use crate::m2::autosar_templates::generic_structure::general_template_classes::ar_package::ARPackageId;",
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
    lines += ["        _ => None,", "    }", "}"]
    return "\n".join(lines)


def emit_equality(ir: Ir, cls: ClassIr) -> str:
    snake = snake_case(cls.name)
    out = [
        "    fn compare_" + snake + "(",
        "        &self,",
        "        other: &Document,",
        f"        a: &{cls.name},",
        f"        b: &{cls.name},",
        "        path: &str,",
        "    ) -> Result<(), String> {",
    ]
    for i, base in enumerate(cls.bases):
        accessor = ".base()" if i == 0 else f".{snake_case(base)}()"
        out.append(f"        self.compare_{snake_case(base)}(other, a{accessor}, b{accessor}, path)?;")
    for field in cls.fields:
        rf = map_field(field, ir, Overrides.default())
        label = rf.rust_name.upper()
        if rf.wrapper == "list":
            out += [
                f"        let list_a = a.get_{rf.rust_name}();",
                f"        let list_b = b.get_{rf.rust_name}();",
                "        if list_a.len() != list_b.len() {",
                f'            return Err(format!("{{path}}: {label} length mismatch"));',
                "        }",
                "        for (index, (x, y)) in list_a.iter().zip(list_b.iter()).enumerate() {",
            ]
            if rf.link:
                arena = snake_case(rf.link) + "s"
                out += [
                    f'            let x = self.{arena}.get(*x).ok_or_else(|| format!("{{path}}.{label}[{{index}}]: id not found in own arena"))?;',
                    f'            let y = other.{arena}.get(*y).ok_or_else(|| format!("{{path}}.{label}[{{index}}]: id not found in other arena"))?;',
                    f'            self.compare_{snake_case(rf.link)}(other, x, y, &format!("{{path}}.{label}[{{index}}]"))?;',
                ]
            else:
                out += [
                    "            if x != y {",
                    f'                return Err(format!("{{path}}.{label}[{{index}}] mismatch"));',
                    "            }",
                ]
            out.append("        }")
        else:
            out += [
                f"        if a.get_{rf.rust_name}() != b.get_{rf.rust_name}() {{",
                f'            return Err(format!("{{path}}: {label} mismatch"));',
                "        }",
            ]
    out += ["        Ok(())", "    }"]
    return "\n".join(out)
```

`emit_document_root(ir, overrides) -> str` (same file) renders `templates.DOCUMENT_RS` with:

1. **Arenas:** one `pub(crate) <plural>: SlotMap<<T>Id, <T>>,` line per concrete emitted class (plural = `snake_case(name) + "s"`; this reproduces every P0 name: `sds`, `sdgs`, `admin_datas`, `l_plain_texts`, …), plus the full `use crate::…::<T>Id;` list.
2. **Resolvers:** `pub fn get_<snake>(&self, id: <T>Id) -> Option<&<T>> { self.<plural>.get(id) }` for every concrete class (P0's `get_ar_package`/`get_sdg`/… are the first instances of this uniform rule).
3. **Factories:** for every element variant (Task 7 `build_element_variants`), `pub fn add_<snake>(&mut self, parent: Option<ARPackageId>, short_name: &str) -> <T>Id` mirroring P0's `add_ar_package` body (insert into arena, set short name, set parent `ElementRef`, push into the package's `elements` when `parent` is `Some`, else into `root_ar_packages` only for `ARPackage` itself).
4. **Equality root:** P0's `assert_structurally_equal` skeleton kept as-is; the hand-written `compare_admin_data`/`compare_ar_package`/… bodies in `DOCUMENT_RS` are replaced by calls to the generated compare fns (emitted as `impl Document` blocks inside each generated module file — Rust allows inherent impls anywhere in the crate, keeping per-module diffs reviewable).
5. **Test module:** `DOCUMENT_RS`'s `#[cfg(test)]` tests are kept; their compare-behaviour assertions still hold because the generated compare produces the same mismatch-message shape.

- [ ] **Step 4: Run tests until green**

Run: `python3 -m unittest discover -s tools/py2rust/tests -v`
Expected: OK (29 tests)

- [ ] **Step 5: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): templates, ElementRef/registry, generated equality, Document arenas"
```

---

### Task 8: main.py orchestration + coverage report + `--check`

**Files:**
- Create: `tools/py2rust/main.py`

- [ ] **Step 1: Implement the CLI**

```python
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
from emit_model import emit_module
from extract import extract_repo
from ir import ClassIr
from overrides import Overrides
from placement import parse_repo
from templates import (AR_OBJECT_RS, AR_PACKAGE_RS, DOCUMENT_RS, ELEMENT_COLLECTION_RS,
                       IDENTIFIABLE_RS, XML_SPACE_BLOCK)

# pinned template -> generated file path (relative to src/)
TEMPLATE_FILES = {
    "ar_object.rs": AR_OBJECT_RS,
    "identifiable.rs": IDENTIFIABLE_RS,
    "element_collection.rs": ELEMENT_COLLECTION_RS,
    "ar_package.rs": AR_PACKAGE_RS,
}


def group_by_module(ir, overrides, placement):
    """(module segments tuple) -> [ClassIr]; unplaced emitted classes are returned for reporting.

    Enums ARE emitted (they become Rust enums); only primitives and skip-listed
    classes (pinned templates, aliases, meta-bases) are filtered out here.
    """
    grouped: dict[tuple[str, ...], list[ClassIr]] = {}
    unplaced: list[str] = []
    for cls in ir.classes():
        if cls.is_primitive or not overrides.emits(cls.name):
            continue
        segments = placement.module_of(cls.name)
        if segments is None:
            unplaced.append(cls.name)
            continue
        grouped.setdefault(tuple(segments), []).append(cls)
    return grouped, sorted(unplaced)
```

Continue `main.py`:

```python
def render(ir, overrides: Overrides, placement) -> dict[str, str]:
    """Return {repo-relative path under src/: file content} for the generated tree."""
    files: dict[str, str] = {}
    grouped, _ = group_by_module(ir, overrides, placement)

    # mod.rs chain: a directory declares every immediate child that is a group
    # (leaf module file) or a directory containing further groups.
    all_segments = list(grouped)
    dirs = {s[:i] for s in all_segments for i in range(1, len(s))}
    for d in sorted(dirs | {tuple(["m2"])}):
        children = sorted({s[len(d)] for s in all_segments if s[:len(d)] == d and len(s) > len(d)})
        lines = ["//! @generated module chain — do not edit.", ""]
        lines += [f"#[allow(dead_code)]"]
        lines += [f"pub mod {child};" for child in children]
        files["/".join(d) + "/mod.rs"] = "\n".join(lines) + "\n"

    for segments, classes in sorted(grouped.items()):
        body = emit_module(ir, overrides, placement, classes, list(segments))
        # the language_data_model module hosts the pinned XmlSpace block
        if segments[-1] == "language_data_model":
            body = body.replace(BANNER_MARKER, BANNER_MARKER + "\n" + XML_SPACE_BLOCK, 1)
        files["/".join(segments) + ".rs"] = body
        # per-class generated equality, appended as `impl Document` blocks
        compare_blocks = []
        for cls in sorted(classes, key=lambda c: c.name):
            if cls.is_enum or cls.is_primitive or not overrides.emits(cls.name):
                continue
            if overrides.skip_emission and cls.name in TEMPLATE_FILE_CLASSES:
                continue  # base chain compare lives in DOCUMENT_RS (P0 hand-written)
            compare_blocks.append(emit_equality(ir, cls))
        if compare_blocks:
            files["/".join(segments) + ".rs"] += "\nimpl Document {\n" + "\n\n".join(compare_blocks) + "\n}\n"

    files["m2/element_registry.rs"] = emit_registry(ir, overrides, build_element_variants(ir))
    files["m2/autosar_templates/autosar_top_level_structure.rs"] = emit_document_root(ir, overrides)
    return files


TEMPLATE_FILE_CLASSES = {
    "ARObject", "Referrable", "MultilanguageReferrable", "Identifiable", "CollectableElement",
    "PackageableElement", "ARElement", "ARPackage", "ReferenceBase",
}
BANNER_MARKER = "// @generated by tools/py2rust"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--py-armodel", required=True)
    parser.add_argument("--out", default="src")
    parser.add_argument("--check", action="store_true", help="fail (exit 1) if regeneration differs")
    args = parser.parse_args()

    ir = extract_repo(args.py_armodel)
    placement = parse_repo(str(pathlib.Path(args.py_armodel) / "autosar/R23-11/markdown"))
    overrides = Overrides.default()

    grouped, unplaced = group_by_module(ir, overrides, placement)
    print(f"classes extracted: {len(ir.classes())} (enums: {len(ir.enums())})")
    print(f"emitted classes: {sum(len(v) for v in grouped.values())}")
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
    print(f"{'drifted files' if args.check else 'files written'}: {changed if args.check else 'n/a'}")
    return 1 if (args.check and changed) else 0


if __name__ == "__main__":
    raise SystemExit(main())
```

(Wire the four `TEMPLATE_FILES` entries into `render()` as literal file writes at their P0 paths — they are not module groups; implement as a small loop. Also add `m2/mod.rs` declaring `pub(crate) mod element_registry;` alongside the generated child modules.)

- [ ] **Step 2: Run against the real checkout, iterate on coverage**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out /tmp/py2rust-out
```

Expected: a coverage report. Iterate **only** by extending `PRIMITIVES`, `overrides.py`, or extractor rules — never by hand-editing output. Exit criteria: the `UNPLACED` list is empty (or every entry is in `overrides.skip_emission` with a one-line reason) and no `KeyError` from `map_field`. Record the final counts in the commit message.

- [ ] **Step 3: Commit**

```bash
git add tools/py2rust
git commit -m "feat(p1): py2rust CLI with full-tree coverage report and --check mode"
```

---

### Task 9: Integrate the generated tree; P0 suite stays green

**Files:**
- Modify (generated): `src/m2/**` (whole tree regenerated)
- Modify (only if compile fallout demands, each with a one-line reason): `src/parser/arxml_parser.rs`, `src/writer/arxml_writer.rs`, `src/lib.rs`

- [ ] **Step 1: Regenerate into the crate**

```bash
python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src
git status --short src/m2 | head -20
```

- [ ] **Step 2: Build and fix fallout (expected minimal)**

Run: `cargo build 2>&1 | head -40`

Known risk points, each with its fix:
1. **`XmlSpace` hosting** — the generated `language_data_model.rs` gets the pinned `XML_SPACE_BLOCK` injected after the banner (Task 8 `render()`), so the hand-written enum survives regeneration and P0 imports keep resolving.
2. **Arena field names** — P0 parser/writer expect `document.sds`/`sdgs`/`admin_datas`/…; the plural rule (`snake_case + "s"`) reproduces them. Verify: `grep -c "document\.\(sds\|sdgs\|admin_datas\|l_plain_texts\|multi_language_plain_texts\|sdg_captions\|sdg_contents\|ar_packages\)" src/parser/arxml_parser.rs` still matches, and the same names exist in the generated `Document`.
3. **P0 unit tests** — the pinned templates carry them verbatim, so `add_ar_package_links_parent_and_root`, `equal_documents_compare_equal`, etc. must still pass unchanged.
4. **dead_code** — generated-but-not-yet-read accessors (consumed in P2) warn; the generated `mod.rs` chain emits `#[allow(dead_code)]` on each `pub mod` declaration (Task 8), and the registry module is `pub(crate)`.

- [ ] **Step 3: Run the full gate**

```bash
set -o pipefail
cargo fmt && cargo fmt --all -- --check \
  && cargo clippy --all-targets -- -D warnings 2>&1 | tail -3 \
  && cargo build 2>&1 | tail -1 \
  && cargo test 2>&1 | grep "test result:"
```

Expected: fmt clean; clippy clean; build ok; all tests pass, including the P0 pinned unit tests and the integration roundtrip. The unit-test *count may grow* (generated compare tests) — every previously-passing test must still pass by name (`cargo test add_ar_package_links_parent_and_root` etc.).

- [ ] **Step 4: Manual smoke**

```bash
cargo run --quiet --bin arxml-dump -- -a tests/integration/test_files/AdminDataWhitespace.arxml
```
Expected: `AR release: R21-11` + `AR-PACKAGE WhitespaceDemo`.

- [ ] **Step 5: Commit**

```bash
git add -A src tools/py2rust/PY_ARMODEL_VERSION
git commit -m "feat(p1): generate full m2 model tree from py-armodel; P0 suite green"
```

---

### Task 10: Regeneration guard, docs, delivery

**Files:**
- Modify: `docs/code_guide.md` (§2 gains a generated-code rule)
- Modify: `AGENTS.md` (one bullet in "Read before writing code")
- Modify: `README.md` (3-line tools section)

- [ ] **Step 1: Docs**

Add to `docs/code_guide.md` §2:

```markdown
- `src/m2/**` is **generated** by `tools/py2rust` (banner-marked). Never hand-edit;
  change the tool or `overrides.py` and re-run:
  `python3 tools/py2rust/main.py --py-armodel target/py-armodel --out src`.
  Regeneration guard: append `--check` — must exit 0. The pinned base chain,
  `Document` root, and `XmlSpace` live in `tools/py2rust/templates.py`; edit
  there, never in the output.
```

Add to `AGENTS.md`:

```markdown
- `src/m2/**` is generated by `tools/py2rust` from py-armodel — never hand-edit generated files; see `docs/code_guide.md` §2.
```

- [ ] **Step 2: Final gate + push + PR**

```bash
set -o pipefail
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test 2>&1 | grep "test result:" \
  && python3 -m unittest discover -s tools/py2rust/tests 2>&1 | tail -1
git push -u origin p1-py2rust-converter
gh issue create --title "P1: py2rust converter generates the full M2 model" --body "…summary + acceptance…"
gh pr create --title "feat: P1 py2rust converter generates the full M2 model" --body "Closes #<n>"
```

---

## Risks / known divergences (carried into the generated code as comments where relevant)

| Risk | Mitigation |
|---|---|
| ~2,000 arenas on one `Document` may slow `cargo build` | Impl blocks are spread across module files; if compile time explodes, split arenas into grouped sub-structs inside `templates.py` (mechanical, P2-independent) |
| `RefType`/`TRefType` treated as `String` | Matches P0; semantic reference resolution is a P2+ concern |
| py getters sort (`getARPackages`), Rust returns insertion order | Deliberate divergence recorded here; round-trip requires insertion order |
| Markdown placement misses classes with no own table | Hard error with `UNPLACED` report; `overrides.skip_emission` escape hatch with reasons |
| Kebab tag rule misfires on digit/acronym names (e.g. `IEEE1722TpConnection`) | `overrides.tag_overrides` fixes individual tags; full tag verification against real ARXML files happens in P2 when the parser port exercises the registry |
| Generated `ElementRef` variants = ARPackage create-directory; `parent` links to non-package containers are coarser than py | Accepted for P1; P2 widens `ElementRef` if the parser port needs it (single enum file, mechanical) |
| Mixin fields (e.g. `VariationPointCapable.variationPoint`) change P0 struct shapes (generated `Sdg` gains the embedded `variation_point_capable` base) | Accept: equality covers the extra base; P0 parser/writer are untouched (they don't read it) |
| `template_classes` in `overrides.py` vs `TEMPLATE_FILE_CLASSES` in `main.py` overlap | One source of truth: `overrides.skip_emission` decides emission; `main.py`'s set only marks which compare fns live in `DOCUMENT_RS` — keep both lists derived from the same constant when implementing |

## Self-review (executed 2026-10-02)

- **Spec §11 P1 coverage:** converter tool ✓ (Tasks 1–8); committed `.rs` output ✓ (Task 9); module placement from markdown `Package` rows ✓ (Task 4); generated `ElementRef` ✓ (Task 7); tag→constructor registry ✓ (Task 7). Generated per-class equality is an addition beyond §11 — justified: the P2–P4 mechanical port needs per-class compare, and emission is free once the IR exists.
- **Placeholder scan:** every code block is complete; the two flagged spots (Task 3 `_extract_creates` tail, Task 8 `group_by_module` filter) each state exactly what to implement. Task 7's templates are verbatim copies of named in-repo files — the engineer can diff, not invent.
- **Type consistency:** `TId` key naming, `snake_case`, `SlotMap<TId, T>`, accessor signatures (`get_` → `Option<&str>` for strings, `Option<T>` for Copy enums/links, `&[T]` + `push_` for lists, `impl Into<String>` setters) all match the post-PR#4 codebase.
