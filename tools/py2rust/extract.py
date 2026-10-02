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
        if not cls.is_enum and not cls.is_primitive:
            cls.fields = _extract_fields(node, source_lines)
            cls.creates = _extract_creates(node)
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
