//! quick-xml `Writer` + element emitters shared by `arxml_writer.rs`
//! (P0 design §9 step 5). Mirrors py's `abstract_arxml_writer.py`.

use std::io::{self, Write};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use thiserror::Error;

use crate::m2::autosar_templates::autosar_top_level_structure::Document;
use crate::m2::autosar_templates::generic_structure::general_template_classes::primitive_types::{
    Limit, RefType, RefTypeId, TRefType,
};

/// Error model for the writer (mirrors `ParseError`).
#[derive(Debug, Error)]
pub enum WriteError {
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("failed schema validation with {count} error(s) (first: line {line}, col {column}: {message})")]
    SchemaValidation {
        count: usize,
        line: usize,
        column: usize,
        message: String,
    },
}

/// Emits `<name attrs>text</name>` on one line. Empty text (`None` or `""`)
/// follows py's net serialization rule (minidom self-closes, then `patch_xml`
/// expands only attribute-less empties): an empty element **with attributes**
/// is self-closing (`<L-2 L="EN"/>`), one **without attributes** is expanded
/// (`<TAG></TAG>`). The empty inline `Text` event is what keeps the end tag
/// on the same line under `new_with_indent`.
pub(crate) fn write_text_element<W: Write>(
    writer: &mut Writer<W>,
    name: &str,
    element: BytesStart<'_>,
    text: Option<&str>,
) -> Result<(), WriteError> {
    let empty = text.is_none_or(str::is_empty);
    if empty && element.attributes().count() > 0 {
        writer.write_event(Event::Empty(element))?;
        return Ok(());
    }
    writer.write_event(Event::Start(element))?;
    match text {
        Some(value) => {
            writer.write_event(Event::Text(BytesText::from_escaped(escape_text(value))))?;
        }
        None => writer.write_event(Event::Text(BytesText::from_escaped("")))?,
    }
    writer.write_event(Event::End(BytesEnd::new(name)))?;
    Ok(())
}

/// py's text escaping (ElementTree `_escape_cdata` + minidom `_write_data`):
/// `&`, `<`, `>`, `"` are escaped, the apostrophe stays raw. quick-xml's
/// `BytesText::new` would additionally emit `&apos;` (e.g. "Young's" in the
/// Unit_Standard fixture), so text goes through `from_escaped` with this
/// function applied instead.
fn escape_text(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
    }
    out
}

/// py `setChildElementOptionalLiteral` — nothing is emitted when None.
pub(crate) fn write_optional_text_element<W: Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    value: Option<&str>,
) -> Result<(), WriteError> {
    if let Some(value) = value {
        write_text_element(writer, tag, BytesStart::new(tag), Some(value))?;
    }
    Ok(())
}

/// py `setChildElementOptionalBooleanValue` — the Boolean's text is the
/// "true"/"false" the reader captured.
pub(crate) fn write_optional_boolean_element<W: Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    value: Option<bool>,
) -> Result<(), WriteError> {
    match value {
        Some(true) => write_optional_text_element(writer, tag, Some("true")),
        Some(false) => write_optional_text_element(writer, tag, Some("false")),
        None => Ok(()),
    }
}

/// py `setChildElementOptionalRefType` — BASE, then DEST, then text value.
pub(crate) fn write_optional_ref_type<W: Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    tag: &str,
    r#ref: Option<&RefType>,
) -> Result<(), WriteError> {
    if let Some(r#ref) = r#ref {
        let mut element = BytesStart::new(tag);
        if let Some(base) = r#ref.get_base() {
            element.push_attribute(("BASE", base));
        }
        if let Some(dest) = r#ref.get_dest() {
            element.push_attribute(("DEST", dest));
        }
        write_text_element(writer, tag, element, r#ref.get_value())?;
    }
    Ok(())
}

/// py `setChildLimitElement` — S/T attrs, INTERVAL-TYPE attr, text.
/// `allow(dead_code)` until Task 3 wires the Compu arms.
#[allow(dead_code)]
pub(crate) fn write_limit_element<W: Write>(
    writer: &mut Writer<W>,
    key: &str,
    limit: &Limit,
) -> Result<(), WriteError> {
    let mut element = BytesStart::new(key);
    if let Some(checksum) = limit.get_checksum() {
        element.push_attribute(("S", checksum));
    }
    if let Some(timestamp) = limit.get_timestamp() {
        element.push_attribute(("T", timestamp));
    }
    if let Some(interval) = limit.get_interval_type() {
        element.push_attribute(("INTERVAL-TYPE", interval));
    }
    write_text_element(writer, key, element, limit.get_value())?;
    Ok(())
}

/// py `setChildElementOptionalRefType` for TRefType-typed elements —
/// identical wire form (BASE, DEST, text).
pub(crate) fn write_optional_t_ref_type<W: Write>(
    writer: &mut Writer<W>,
    tag: &str,
    r#ref: Option<&TRefType>,
) -> Result<(), WriteError> {
    if let Some(r#ref) = r#ref {
        let mut element = BytesStart::new(tag);
        if let Some(base) = r#ref.get_base() {
            element.push_attribute(("BASE", base));
        }
        if let Some(dest) = r#ref.get_dest() {
            element.push_attribute(("DEST", dest));
        }
        write_text_element(writer, tag, element, r#ref.get_value())?;
    }
    Ok(())
}

/// py's `ET.SubElement(wrapper)` + per-item `setChildElementOptionalRefType`
/// — the wrapper is emitted only when the list is non-empty.
pub(crate) fn write_ref_type_list<W: Write>(
    writer: &mut quick_xml::writer::Writer<W>,
    wrapper: &str,
    tag: &str,
    refs: &[RefTypeId],
    document: &Document,
) -> Result<(), WriteError> {
    if refs.is_empty() {
        return Ok(());
    }
    writer.write_event(Event::Start(BytesStart::new(wrapper)))?;
    for ref_id in refs {
        if let Some(r#ref) = document.ref_types.get(*ref_id) {
            write_optional_ref_type(writer, tag, Some(r#ref))?;
        }
    }
    writer.write_event(Event::End(BytesEnd::new(wrapper)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_element_expands_empty_elements_inline() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let element = BytesStart::new("LANGUAGE");
        write_text_element(&mut writer, "LANGUAGE", element, None).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), "<LANGUAGE></LANGUAGE>");
    }

    /// py `patch_xml` expands only attribute-less empty elements — one with
    /// attributes stays self-closing (`<L-2 L="EN"/>`).
    #[test]
    fn empty_element_with_attributes_self_closes() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let mut element = BytesStart::new("L-2");
        element.push_attribute(("L", "EN"));
        write_text_element(&mut writer, "L-2", element, Some("")).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(String::from_utf8(buffer).unwrap(), "<L-2 L=\"EN\"/>");
    }

    #[test]
    fn text_element_escapes_and_writes_text() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let element = BytesStart::new("SHORT-NAME");
        write_text_element(&mut writer, "SHORT-NAME", element, Some("a<b")).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "<SHORT-NAME>a&lt;b</SHORT-NAME>"
        );
    }

    #[test]
    fn text_element_keeps_apostrophes_raw_like_py() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let element = BytesStart::new("L-1");
        write_text_element(&mut writer, "L-1", element, Some("Young's \"Pascal\"")).unwrap();
        writer.write_event(Event::Eof).unwrap();
        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            "<L-1>Young's &quot;Pascal&quot;</L-1>"
        );
    }
}
