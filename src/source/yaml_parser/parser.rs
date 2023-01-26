extern crate yaml_rust;

use std::error::Error;

use yaml_rust::{Yaml, YamlLoader};

use crate::source::metadata::{HttpMethod, Metadata, MetadataParser, API};
use crate::source::open_api::{OpenAPI, Path};
use crate::source::MINE;

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

        let api_list=open_api
            .paths
            .into_iter()
            .map(|Path(url, method, operation)| API {
                url,
                method: HttpMethod(method),
                summary: operation.summary,
                produces: operation
                    .produces
                    .into_iter()
                    .map(|string| MINE(string))
                    .collect(),
            })
            .collect();

        Ok(Metadata {
            base_path: open_api.base_path,
            api_list,
            model_list: vec![],
        })
    }
}

pub fn read_base_path(doc: &Yaml) -> Option<String> {
    doc.as_str().and_then(|str| Some(str.to_string()))
}

pub fn read_mine_type(doc: &Yaml) -> Option<Vec<String>> {
    doc.as_vec().and_then(|vec| {
        Some(
            vec.into_iter()
                .map(|item| item.as_str().unwrap_or("text/html").to_string())
                .collect(),
        )
    })
}

pub fn read_hash_value<'a>(hash_key: &'a Yaml) -> impl Fn(&'a Yaml) -> Option<&'a Yaml> {
    |yaml: &Yaml| yaml.as_hash().and_then(|hash| hash.get(hash_key))
}

pub fn read_summary(yaml: &Yaml) -> Option<String> {
    yaml.as_str().and_then(|s| Some(s.to_string()))
}
