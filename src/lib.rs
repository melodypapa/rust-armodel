pub mod m2;
pub mod reader;
pub mod transformer;
pub mod validation;
pub mod writer;

pub use m2::autosar_templates::autosar_top_level_structure::Document;
pub use reader::abstract_arxml_reader::ParseError;
pub use reader::arxml_reader::{default_options, ARXMLReader, ReaderOptions};
pub use transformer::admin_data::AdminDataTransformer;
pub use writer::abstract_arxml_writer::WriteError;
pub use writer::arxml_writer::{ARXMLWriter, WriterOptions};
