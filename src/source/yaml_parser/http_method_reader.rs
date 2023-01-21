use crate::source::metadata_context::{HttpMethod, MINE};
use yaml_rust::Yaml;

impl HttpMethod {
    pub fn from_yaml_doc(doc: &Yaml) -> Vec<(String, Self)> {
        let method_hash = doc.as_hash().expect("read methods failed");
        let methods: [&str; 5] = ["get", "post", "patch", "put", "delete"];

        let mut method_list = vec![];
        for method_str in methods {
            let method_name = method_str.to_string();
            let method_key = &Yaml::String(method_name.clone());
            let summary_key = &Yaml::String(String::from("summary"));
            let produces_key = &Yaml::String(String::from("produces"));

            if !method_hash.contains_key(method_key) {
                continue;
            }

            let method = method_hash.get(method_key);

            let summary = method
                .and_then(read_hash_value(summary_key))
                .and_then(read_summary)
                .unwrap_or(String::from(""));
            let produces = method
                .and_then(read_hash_value(produces_key))
                .and_then(read_produces)
                .unwrap_or(vec![]);

            method_list.push((method_name, HttpMethod { summary, produces }))
        }

        method_list
    }
}

fn read_hash_value<'a>(hash_key: &'a Yaml) -> impl Fn(&'a Yaml) -> Option<&'a Yaml> {
    |yaml: &Yaml| yaml.as_hash().and_then(|hash| hash.get(hash_key))
}

fn read_produces(yaml: &Yaml) -> Option<Vec<MINE>> {
    yaml.as_vec().and_then(|vec| {
        Some(
            vec.into_iter()
                .map(|item| MINE(item.as_str().unwrap_or("text/html").to_string()))
                .collect(),
        )
    })
}

fn read_summary(yaml: &Yaml) -> Option<String> {
    yaml.as_str().and_then(|s| Some(s.to_string()))
}
