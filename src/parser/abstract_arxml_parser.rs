//! Node DOM + `find`/`find_all`/`get_child_element_string` helpers
//! (P0 design §6). Mirrors py's `abstract_arxml_parser.py`.

use std::collections::BTreeMap;
use std::io::BufRead;
use std::path::Path;

use quick_xml::escape::unescape;
use quick_xml::events::attributes::AttrError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::XmlVersion;
use thiserror::Error;

/// Error model mirroring py's raise/notImplemented split
/// (`docs/code_guide.md` §7).
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected root element `{0}`")]
    UnexpectedRoot(String),
    #[error("invalid {element}: {reason}")]
    InvalidElement { element: String, reason: String },
    #[error("invalid attribute: {0}")]
    Attr(#[from] AttrError),
    #[error(transparent)]
    Xml(#[from] quick_xml::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A minimal DOM node. Attribute keys are stored as they appear in the
/// document (`GID`, `xml:space`, `xsi:schemaLocation` — prefix kept, matching
/// the wire format); element names are namespace-stripped local names,
/// matching py's `getPureTagName`.
#[derive(Debug, Default, Clone)]
pub struct Node {
    pub name: String,
    pub attrs: BTreeMap<String, String>,
    pub children: Vec<Node>,
    pub text: Option<String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// Whitespace rule (P0 design §6): text is captured verbatim when the element
/// carries `xml:space="preserve"`, dropped when whitespace-only, and trimmed
/// otherwise. This is required for the `SD` value `"special   data"`.
fn apply_whitespace_rule(node: &mut Node) {
    if node.attrs.get("xml:space").map(String::as_str) == Some("preserve") {
        return;
    }
    if let Some(text) = node.text.as_mut() {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            node.text = None;
        } else {
            *text = trimmed.to_string();
        }
    }
}

fn start_to_node(start: &BytesStart<'_>) -> Result<Node, ParseError> {
    let mut node = Node::new(start.local_name().as_ref());
    for attr in start.attributes() {
        let attr = attr?;
        // XML attribute-value normalization (matches ElementTree's semantics).
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
        node.attrs.insert(attr.key.into_inner().to_string(), value);
    }
    Ok(node)
}

fn unescape_text(raw: &str) -> Result<String, ParseError> {
    unescape(raw)
        .map(|value| value.into_owned())
        .map_err(|error| ParseError::InvalidElement {
            element: String::new(),
            reason: format!("cannot unescape text: {error}"),
        })
}

/// Builds the `Node` DOM from a quick-xml reader.
pub fn build_dom_from_reader<R: BufRead>(reader: Reader<R>) -> Result<Node, ParseError> {
    let mut reader = reader;
    reader.config_mut().trim_text(false);

    let mut stack: Vec<Node> = Vec::new();
    let mut root: Option<Node> = None;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(start) => {
                stack.push(start_to_node(&start)?);
            }
            Event::Empty(start) => {
                let mut node = start_to_node(&start)?;
                apply_whitespace_rule(&mut node);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                }
            }
            Event::Text(text) => {
                if let Some(current) = stack.last_mut() {
                    let raw = unescape_text(text.as_ref())?;
                    current.text.get_or_insert_with(String::new).push_str(&raw);
                }
            }
            Event::CData(cdata) => {
                if let Some(current) = stack.last_mut() {
                    current
                        .text
                        .get_or_insert_with(String::new)
                        .push_str(cdata.as_ref());
                }
            }
            Event::GeneralRef(reference) => {
                if let Some(current) = stack.last_mut() {
                    // quick-xml splits entity references out of text; resolve
                    // the predefined/numeric form via `unescape`.
                    let raw = format!("&{};", reference.as_ref());
                    let resolved = unescape_text(&raw)?;
                    current
                        .text
                        .get_or_insert_with(String::new)
                        .push_str(&resolved);
                }
            }
            Event::End(_) => {
                let mut node = stack.pop().ok_or_else(|| ParseError::InvalidElement {
                    element: String::new(),
                    reason: "unbalanced end tag".to_string(),
                })?;
                apply_whitespace_rule(&mut node);
                match stack.last_mut() {
                    Some(parent) => parent.children.push(node),
                    None => {
                        if root.is_some() {
                            return Err(ParseError::InvalidElement {
                                element: node.name,
                                reason: "multiple root elements".to_string(),
                            });
                        }
                        root = Some(node);
                    }
                }
            }
            Event::Eof => break,
            _ => {} // Decl, Comment, PI, DocType — ignored
        }
    }

    if let Some(unclosed) = stack.pop() {
        return Err(ParseError::InvalidElement {
            element: unclosed.name,
            reason: "unclosed element(s) at end of document".to_string(),
        });
    }

    root.ok_or_else(|| ParseError::InvalidElement {
        element: String::new(),
        reason: "document contains no elements".to_string(),
    })
}

/// Builds the `Node` DOM from a file.
pub fn build_dom(path: &Path) -> Result<Node, ParseError> {
    build_dom_from_reader(Reader::from_file(path)?)
}

/// py `AbstractARXMLParser.findall` — supports the `A/B`, `A/*` and `./A`
/// key forms; `*` matches any child.
pub fn find_all<'a>(parent: &'a Node, key: &str) -> Vec<&'a Node> {
    let mut current: Vec<&Node> = vec![parent];
    for segment in key.split('/') {
        let mut next: Vec<&Node> = Vec::new();
        for node in current {
            if segment == "." {
                next.push(node);
            } else if segment == "*" {
                next.extend(node.children.iter());
            } else {
                next.extend(node.children.iter().filter(|child| child.name == segment));
            }
        }
        current = next;
    }
    current
}

/// py `AbstractARXMLParser.find` — first match of `find_all`.
pub fn find<'a>(parent: &'a Node, key: &str) -> Option<&'a Node> {
    find_all(parent, key).into_iter().next()
}

/// py `getChildElementOptionalStringValue`
pub fn get_child_element_string<'a>(parent: &'a Node, key: &str) -> Option<&'a str> {
    find(parent, key).and_then(|node| node.text.as_deref())
}

/// py `getShortName` — required by the spec; missing SHORT-NAME is an error.
pub fn get_short_name(element: &Node) -> Result<String, ParseError> {
    match get_child_element_string(element, "SHORT-NAME") {
        Some(name) => Ok(name.to_string()),
        None => Err(ParseError::InvalidElement {
            element: element.name.clone(),
            reason: "SHORT-NAME is required".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0">
  <SHORT-NAME>Demo</SHORT-NAME>
  <SD GID="purpose" xml:space="preserve">special   data</SD>
  <PLAIN>  padded  </PLAIN>
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>P1</SHORT-NAME>
    </AR-PACKAGE>
    <AR-PACKAGE>
      <SHORT-NAME>P2</SHORT-NAME>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

    fn sample_dom() -> Node {
        build_dom_from_reader(Reader::from_str(SAMPLE)).unwrap()
    }

    #[test]
    fn dom_strips_namespaces_and_keeps_attributes() {
        let root = sample_dom();
        assert_eq!(root.name, "AUTOSAR");
        assert_eq!(
            root.attrs.get("xmlns").map(String::as_str),
            Some("http://autosar.org/schema/r4.0")
        );
    }

    #[test]
    fn whitespace_rule_is_applied_per_element() {
        let root = sample_dom();
        let sd = find(&root, "SD").unwrap();
        assert_eq!(sd.text.as_deref(), Some("special   data"));
        assert_eq!(
            sd.attrs.get("xml:space").map(String::as_str),
            Some("preserve")
        );

        let plain = find(&root, "PLAIN").unwrap();
        assert_eq!(plain.text.as_deref(), Some("padded"));

        let short_name = find(&root, "SHORT-NAME").unwrap();
        assert_eq!(short_name.text.as_deref(), Some("Demo"));
    }

    #[test]
    fn find_and_find_all_support_paths() {
        let root = sample_dom();
        let first = find(&root, "AR-PACKAGES/AR-PACKAGE/SHORT-NAME").unwrap();
        assert_eq!(first.text.as_deref(), Some("P1"));

        let packages = find_all(&root, "AR-PACKAGES/*");
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].name, "AR-PACKAGE");

        assert!(find(&root, "MISSING").is_none());
    }

    #[test]
    fn get_short_name_requires_element() {
        let root = sample_dom();
        assert_eq!(get_short_name(&root).unwrap(), "Demo");

        let error = get_short_name(find(&root, "PLAIN").unwrap()).unwrap_err();
        assert!(error.to_string().contains("SHORT-NAME is required"));
    }
}
