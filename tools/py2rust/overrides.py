"""Hand-maintained P0-parity quirks. Everything not listed here converts mechanically."""
from __future__ import annotations

import re


def kebab_tag(class_name: str) -> str:
    return re.sub(r"(?<=[a-z0-9])([A-Z])|(?<=[A-Z])([A-Z][a-z])", r"-\1\2", class_name).upper()


class Overrides:
    def __init__(self, *, type_alias, skip_emission, tag_overrides, alias_module=None):
        self.type_alias_map = type_alias
        self.skip_emission = skip_emission
        self.tag_overrides = tag_overrides
        # aliased hand-written types are not in placement; this is where they live
        self.alias_module = alias_module or {}

    @staticmethod
    def default() -> "Overrides":
        return Overrides(
            # generated enum name -> existing hand-written Rust type (not regenerated)
            type_alias={
                "XmlSpaceEnum": "XmlSpace",
                # abstract compu content classes → pinned usage-site enums
                # (batch-2 Task 3; the concrete subclasses are arena'd)
                "CompuContent": "CompuContentRef",
                "CompuScaleContents": "CompuScaleContentsRef",
                "CompuConstContent": "CompuConstContentRef",
            },
            # not emitted as Rust: aliased, pinned templates, or primitive meta-bases
            skip_emission={
                "XmlSpaceEnum",                                   # aliased to hand-written XmlSpace
                "ARType", "ARLiteral", "AREnum",                  # primitive meta-bases
                # pinned templates (base chain + root), copied verbatim in templates.py:
                "ARObject", "Referrable", "MultilanguageReferrable", "Identifiable",
                "CollectableElement", "PackageableElement", "ARElement", "ARPackage",
                "ReferenceBase",
                "AUTOSAR", "AbstractAUTOSAR", "AUTOSARDoc", "FileInfoComment",
            },
            # kebab rule misfires on digit/acronym runs; fixed tags verified against py parser in P2
            tag_overrides={},
            alias_module={
                "XmlSpace": "m2::msr::documentation::text_model::language_data_model",
                "CompuContentRef": "m2::msr::asam_hdo::computation_method",
                "CompuScaleContentsRef": "m2::msr::asam_hdo::computation_method",
                "CompuConstContentRef": "m2::msr::asam_hdo::computation_method",
                # the type-erased handle lives in the pinned ar_object.rs template
                "ElementRef": "m2::autosar_templates::generic_structure::general_template_classes::ar_object",
            },
        )

    def type_alias(self, name: str) -> str | None:
        return self.type_alias_map.get(name)

    def emits(self, name: str) -> bool:
        return name not in self.skip_emission

    def tag_for(self, class_name: str) -> str:
        return self.tag_overrides.get(class_name, kebab_tag(class_name))
