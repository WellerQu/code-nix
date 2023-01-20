extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};
use crate::source::metadata_context::{ SourceParser, MetadataContext };
use crate::source::metadata_context::{ API, Model };

#[derive(Debug, PartialEq, Eq)]
pub struct YamlSourceParser {

}

impl YamlSourceParser {
    pub fn create<'a>() -> &'a Self {
        &YamlSourceParser { }
    }
}

impl SourceParser for YamlSourceParser {
    fn get_context_from_str(&self, str: &str) -> MetadataContext {
        let docs = YamlLoader::load_from_str(str)
            .expect("parse yaml failed");
        let doc = &docs[0];

        MetadataContext { 
            base_path: read_base_path(doc),
            api_list: API::from_yaml_doc(doc),
            model_list: Model::from_yaml_doc(doc),
        }
    }
}

fn read_base_path(doc: &Yaml) -> String {
    doc["basePath"]
        .as_str()
        .expect("read basePath failed")
        .to_string()
}