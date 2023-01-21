use crate::source::metadata_context::{HttpMethod, MINE};
use std::collections::HashMap;
use yaml_rust::Yaml;

impl HttpMethod {
    pub fn from_yaml_doc(doc: &Yaml) -> HashMap<String, Self> {
        let mut method_map: HashMap<String, HttpMethod> = HashMap::new();

        let method_schema = doc.as_hash().expect("read methods failed");

        for (method, doc) in method_schema {
            method_map
                .entry(String::from(method.as_str().expect("read method failed")))
                .or_insert(HttpMethod {
                    summary: doc["summary"]
                        .as_str()
                        .and_then(read_summary)
                        .unwrap_or(String::from("")),
                    produces: doc["produces"]
                        .as_vec()
                        .and_then(read_produces)
                        .unwrap_or(vec![]),
                });
        }

        method_map
    }
}

fn read_produces(vec: &Vec<Yaml>) -> Option<Vec<MINE>> {
    let mine_types = vec
        .into_iter()
        .map(|item| MINE(item.as_str().unwrap_or("text/html").to_string()))
        .collect();

    Some(mine_types)
}

fn read_summary(str: &str) -> Option<String> {
    Some(str.to_string())
}
