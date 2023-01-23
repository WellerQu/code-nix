use yaml_rust::{Yaml, yaml::Hash};

pub struct Parameter {}

pub struct Operation {
    pub summary: String,
    // TODO: tags in Operation
    // pub tags: Vec<String>,
    pub produces: Vec<String>,
    pub consumes: Vec<String>,
    pub parameters: Vec<Parameter>,
}

pub struct Path(String, Operation);

impl Path {
    fn from_doc(doc: &Yaml) -> Vec<Self> {
        let empty_hash = Hash::new();
        let operation_hash = doc.as_hash().unwrap_or(&empty_hash);
        let operation_methods: [&str; 5] = ["get", "post", "patch", "put", "delete"];

        let mut operation_list = vec![];
        for method_str in operation_methods {
            let method_name = method_str.to_string();
            let method_key = &Yaml::String(method_name.clone());

            if !operation_hash.contains_key(method_key) {
                continue;
            }

            let summary_key = &Yaml::String(String::from("summary"));
            let produces_key = &Yaml::String(String::from("produces"));
            let consumes_key = &Yaml::String(String::from("consumes"));

            let method = operation_hash.get(method_key);

            let summary = method
                .and_then(read_hash_value(summary_key))
                .and_then(read_summary)
                .unwrap_or(String::from(""));
            let produces = method
                .and_then(read_hash_value(produces_key))
                .and_then(read_mine_type)
                .unwrap_or(vec![]);
            let consumes = method
                .and_then(read_hash_value(consumes_key))
                .and_then(read_mine_type)
                .unwrap_or(vec![]);

            operation_list.push(Path(
                method_name,
                Operation {
                    summary,
                    produces,
                    consumes,
                    parameters: vec![],
                },
            ))
        }

        operation_list
    }
}

// struct Definition {
// }

pub struct OpenAPI {
    pub base_path: String,
    pub produces: Vec<String>,
    pub consumes: Vec<String>,
    pub paths: Vec<Path>,
    // TODO: more fields in OpenAPI
    // pub schemes: Vec<String>,
    // pub host: String,
    // pub definitions: Vec<Definition>,
}

impl OpenAPI {
    pub fn from_doc(doc: &Yaml) -> OpenAPI {
        OpenAPI {
            base_path: read_base_path(&doc["basePath"]).unwrap_or(String::new()),
            produces: read_mine_type(&doc["produces"]).unwrap_or(vec![]),
            consumes: read_mine_type(&doc["consumes"]).unwrap_or(vec![]),
            paths: Path::from_doc(&doc["paths"]),
        }
    }
}

fn read_base_path(doc: &Yaml) -> Option<String> {
    doc.as_str().and_then(|str| Some(str.to_string()))
}

fn read_mine_type(doc: &Yaml) -> Option<Vec<String>> {
    doc.as_vec().and_then(|vec| {
        Some(
            vec.into_iter()
                .map(|item| item.as_str().unwrap_or("text/html").to_string())
                .collect(),
        )
    })
}

fn read_hash_value<'a>(hash_key: &'a Yaml) -> impl Fn(&'a Yaml) -> Option<&'a Yaml> {
    |yaml: &Yaml| yaml.as_hash().and_then(|hash| hash.get(hash_key))
}

fn read_summary(yaml: &Yaml) -> Option<String> {
    yaml.as_str().and_then(|s| Some(s.to_string()))
}
