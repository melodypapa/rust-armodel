"""Intermediate representation shared by extract/emit. Plain dataclasses only."""
from __future__ import annotations

import re
from dataclasses import dataclass, field


RUST_KEYWORDS = {
    "as", "box", "break", "const", "continue", "dyn", "else", "enum", "extern", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await",
}


def snake_case(name: str) -> str:
    """usedLanguages → used_languages; DocRevisions → doc_revisions;
    acronyms stay one word: MSR → msr, AUTOSARTemplates → autosar_templates."""
    s1 = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1 \2", name)
    s2 = re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", s1)
    out = "_".join(part.lower() for part in s2.split())
    if out in RUST_KEYWORDS:
        out = "r#" + out
    return out


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
    has_remover: bool = False       # the defining py class also defines remove<Field>

    @staticmethod
    def rust_name_for(name: str) -> str:
        # py `_vf` -> rust `vf`: a leading underscore reads as the
        # unused-variable convention (get__vf is also non-snake-case)
        return snake_case(name.lstrip("_"))


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
