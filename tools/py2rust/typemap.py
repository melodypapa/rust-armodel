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
        return RustField(FieldIr.rust_name_for(field.name), alias, field.kind, link=None)
    if inner in PRIMITIVES:
        return RustField(FieldIr.rust_name_for(field.name), "String", field.kind, link=None)
    cls = ir.get(inner)
    if cls is None:
        raise KeyError(f"unknown field type {inner!r} (field {field.name}); add to PRIMITIVES or overrides")
    if cls.is_enum:
        return RustField(FieldIr.rust_name_for(field.name), inner, field.kind, link=None)
    if cls.is_primitive:
        return RustField(FieldIr.rust_name_for(field.name), "String", field.kind, link=None)
    return RustField(FieldIr.rust_name_for(field.name), f"{inner}Id", field.kind, link=inner)
