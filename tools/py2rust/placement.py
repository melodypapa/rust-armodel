"""Map spec classes to rust module paths via the markdown | Class | / | Package | rows."""
from __future__ import annotations

import pathlib
import re

from ir import snake_case

CLASS_ROW = re.compile(r"^\|\s*Class\s*\|\s*([A-Za-z][A-Za-z0-9_]*)(\s*\(abstract\))?\s*\|")
PRIMITIVE_ROW = re.compile(r"^\|\s*Primitive\s*\|\s*([A-Za-z][A-Za-z0-9_]*)\s*\|")
ENUMERATION_ROW = re.compile(r"^\|\s*Enumeration\s*\|\s*([A-Za-z][A-Za-z0-9_]*)\s*\|")
LITERAL_ROW = re.compile(r"^\|\s*Literal\s*\|\s*([A-Za-z][A-Za-z0-9_]*)\s*\|")
PACKAGE_ROW = re.compile(r"^\|\s*Package\s*\|\s*(M2::[A-Za-z0-9:]+)\s*\|")


class Placement:
    def __init__(self, class_to_package: dict[str, str]):
        self._by_class = class_to_package

    @staticmethod
    def package_to_segments(package: str) -> list[str]:
        return [snake_case(part) for part in package.split("::")]

    def module_of(self, class_name: str) -> list[str] | None:
        package = self._by_class.get(class_name)
        return Placement.package_to_segments(package) if package else None


def parse_markdown(files: dict[str, str]) -> Placement:
    """files: markdown file name -> content. Tables split across page breaks are
    handled by tracking the most recent Class/Primitive row per file."""
    binding: dict[str, str] = {}
    for content in files.values():
        current: str | None = None
        for line in content.splitlines():
            match = (CLASS_ROW.match(line) or PRIMITIVE_ROW.match(line)
                     or ENUMERATION_ROW.match(line) or LITERAL_ROW.match(line))
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
