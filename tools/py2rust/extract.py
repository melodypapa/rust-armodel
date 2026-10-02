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
