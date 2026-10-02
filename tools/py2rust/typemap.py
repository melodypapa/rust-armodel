"""Map py field annotations to Rust field kinds."""
from __future__ import annotations

from dataclasses import dataclass

from ir import FieldIr, Ir
from overrides import Overrides

# py-armodel scalar primitives (PrimitiveTypes.py, non-enum) -> Rust String.
# Values round-trip verbatim, matching P0 (P0 design §5: NameToken/VerbatimStringPlain → String).
# AREnum subclasses (all *Enum names) are NOT here — they become generated Rust enums.
# Belt-and-braces: the extractor also flags ARLiteral/ARType-derived classes via
# ClassIr.is_primitive, and map_field honours that flag too.
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
    # P0-parity quirk: language codes (AdminData.language, LanguageSpecific.l)
    # are stored as plain String in the P0 model, not as the generated LEnum.
    "LEnum",
}


@dataclass
class RustField:
    rust_name: str
    rust_type: str      # element type as written in Rust: String / <Enum> / <T>Id
    wrapper: str        # "optional" | "list"
    link: str | None    # None for owned scalars/enums; "<T>" for arena links
    has_remover: bool = False  # the py field's class defines remove<Field>


# types referenced by py-armodel but never defined there (upstream placeholders,
# e.g. V2xSupportEnum "Rule 0001.10 placeholder") -> String, reported by main()
UNKNOWN_LOG: list[str] = []


def unknown_types() -> list[str]:
    return sorted(set(UNKNOWN_LOG))


def map_field(field: FieldIr, ir: Ir, overrides: Overrides) -> RustField:
    inner = field.inner
    alias = overrides.type_alias(inner)
    if alias:
        return RustField(FieldIr.rust_name_for(field.name), alias, field.kind, link=None,
                         has_remover=field.has_remover)
    if inner == "ElementRef":
        # type-erased handle (CollectableElement/ARPackage element lists)
        return RustField(FieldIr.rust_name_for(field.name), "ElementRef", field.kind, link=None,
                         has_remover=field.has_remover)
    if inner in PRIMITIVES or inner in ("str", "int", "float", "bool", "Any"):
        return RustField(FieldIr.rust_name_for(field.name), "String", field.kind, link=None,
                         has_remover=field.has_remover)
    cls = ir.get(inner)
    if cls is None:
        UNKNOWN_LOG.append(inner)
        return RustField(FieldIr.rust_name_for(field.name), "String", field.kind, link=None,
                         has_remover=field.has_remover)
    if cls.is_enum:
        return RustField(FieldIr.rust_name_for(field.name), inner, field.kind, link=None,
                         has_remover=field.has_remover)
    if cls.is_primitive:
        return RustField(FieldIr.rust_name_for(field.name), "String", field.kind, link=None,
                         has_remover=field.has_remover)
    if cls.is_abstract:
        # abstract targets have no arena; P0 models such links as the type-erased
        # ElementRef handle (P0 design §5: ElementRef replaces py's untyped parent)
        return RustField(FieldIr.rust_name_for(field.name), "ElementRef", field.kind, link=None,
                         has_remover=field.has_remover)
    return RustField(FieldIr.rust_name_for(field.name), f"{inner}Id", field.kind, link=inner,
                     has_remover=field.has_remover)
