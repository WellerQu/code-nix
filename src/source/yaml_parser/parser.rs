extern crate yaml_rust;

use std::error::Error;

use yaml_rust::YamlLoader;

use crate::source::metadata::{Metadata, MetadataParser};
use crate::source::yaml_parser::open_api::OpenAPI;

#[derive(Debug, PartialEq, Eq)]
pub struct YamlDocParser {}

impl YamlDocParser {
    pub fn new<'a>() -> &'a Self {
        &YamlDocParser {}
    }
}

impl MetadataParser for YamlDocParser {
    fn from_str(&self, str: &str) -> Result<Metadata, Box<dyn Error>> {
        let docs = YamlLoader::load_from_str(str)?;
        let doc = &docs[0];

        let open_api = OpenAPI::from_doc(doc);

        Ok(Metadata {
            base_path: open_api.base_path,
            api_list: vec![],
            model_list: vec![],
        })
    }
}
