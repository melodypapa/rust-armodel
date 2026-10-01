pub mod m2;
pub mod parser;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
pub use parser::abstract_arxml_parser::ParseError;
pub use parser::arxml_parser::{default_options, ARXMLParser, ParserOptions};
