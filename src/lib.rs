pub mod m2;
pub mod parser;
pub mod transformer;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
pub use parser::abstract_arxml_parser::ParseError;
pub use parser::arxml_parser::{default_options, ARXMLParser, ParserOptions};
pub use transformer::admin_data::AdminDataTransformer;
pub use writer::abstract_arxml_writer::WriteError;
pub use writer::arxml_writer::{ARXMLWriter, WriterOptions};
