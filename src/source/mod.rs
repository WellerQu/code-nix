pub(crate) mod yaml_parser;
pub(crate) mod metadata_context;

pub use crate::source::metadata_context::{ SourceParser, MetadataContext, API, HttpMethod, MINE };
pub use crate::source::yaml_parser::source_parser::YamlSourceParser;

pub enum ParserType {
    YamlParser,
}

pub fn create_parser<'a>(pt: ParserType) -> &'a impl SourceParser {
    match pt {
        ParserType::YamlParser => YamlSourceParser::create(),
    }
}