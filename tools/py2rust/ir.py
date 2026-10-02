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
