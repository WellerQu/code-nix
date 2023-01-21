use yaml_rust::{Yaml};
use yaml_rust::yaml::Hash;
use crate::source::metadata_context::{ API, HttpMethod };

impl API {
    pub fn from_yaml_doc(doc: &Yaml) -> Vec<Self> {
        let empty_api_pathes = Hash::new();
        let api_pathes = doc["paths"]
            .as_hash()
            .unwrap_or(&empty_api_pathes);

        let mut apis: Vec<API> = vec![];

        for (url, sub_schema) in api_pathes {
            apis.push(
                API {
                    url: url.as_str().expect("read api url failed").to_string(),
                    methods: HttpMethod::from_yaml_doc(sub_schema),
                }
            )
        }

        apis
    }
}