pub(crate) mod yaml_parser;
pub(crate) mod metadata;

pub use crate::source::metadata::{ MetadataParser, Metadata, API, HttpMethod, MINE };

use crate::source::yaml_parser::parser::YamlDocParser;

pub enum ParserType {
    YamlParser,
}

pub fn create_parser<'a>(pt: ParserType) -> &'a impl MetadataParser {
    match pt {
        ParserType::YamlParser => YamlDocParser::new()
    }
}