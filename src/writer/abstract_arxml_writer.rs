//! quick-xml `Writer` + element emitters shared by `arxml_writer.rs`
//! (P0 design §9 step 5). Mirrors py's `abstract_arxml_writer.py`.

use std::io::{self, Write};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use thiserror::Error;

/// Error model for the writer (mirrors `ParseError`).
#[derive(Debug, Error)]
pub enum WriteError {
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// Emits `<name attrs>text</name>` on one line; when `text` is `None` the
/// element is still expanded (`<name></name>`, not self-closing) to match
/// py's `short_empty_elements=False`. The empty inline `Text` event is what
/// keeps the end tag on the same line under `new_with_indent`.
pub(crate) fn write_text_element<W: Write>(
    writer: &mut Writer<W>,
    name: &str,
    element: BytesStart<'_>,
    text: Option<&str>,
) -> Result<(), WriteError> {
    writer.write_event(Event::Start(element))?;
    match text {
        Some(value) => writer.write_event(Event::Text(BytesText::new(value)))?,
        None => writer.write_event(Event::Text(BytesText::from_escaped("")))?,
    }
    writer.write_event(Event::End(BytesEnd::new(name)))?;
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
}
