use yaml_rust::{yaml::Hash, Yaml};

use crate::source::yaml_parser::parser::{
    read_base_path, read_hash_value, read_mine_type, read_summary,
};
use crate::{string, yaml_string};

pub struct ParameterSchema {
    pub param_in: String,
    pub param_name: String,
    pub param_type: String,
    pub param_required: bool,
    pub schema: Option<Box<ParameterSchema>>,
}

pub struct Parameter {
    pub ref_path: Option<String>,
    pub schema: Option<Box<ParameterSchema>>,
}

impl Parameter {
    fn apply_ref(&mut self) {
        self.schema = Some(Box::new(ParameterSchema {
            schema: None,
            param_in: String::new(),
            param_name: String::new(),
            param_required: false,
            param_type: String::new(),
        }));
    }
}

pub struct Operation {
    pub summary: String,
    // TODO: tags in Operation
    // pub tags: Vec<String>,
    pub produces: Vec<String>,
    pub consumes: Vec<String>,
    pub parameters: Vec<Parameter>,
}

pub struct Path(pub String, pub String, pub Operation);

impl Path {
    fn from_doc(doc: &Yaml) -> Vec<Self> {
        let paths_hash = doc.as_hash().expect("read paths failed");
        let mut operation_list = vec![];

        for (url, schema) in paths_hash {
            let empty_hash = Hash::new();
            let operation_hash = schema.as_hash().unwrap_or(&empty_hash);
            let operation_methods: [&str; 5] = ["get", "post", "patch", "put", "delete"];

            for method_str in operation_methods {
                let method_name = method_str.to_string();
                let method_key = &Yaml::String(method_name.clone());

                if !operation_hash.contains_key(method_key) {
                    continue;
                }

                let summary_key = &yaml_string!("summary");
                let produces_key = &yaml_string!("produces");
                let consumes_key = &yaml_string!("consumes");

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
                    string!(url.as_str().expect("read url failed")),
                    method_name,
                    Operation {
                        summary,
                        produces,
                        consumes,
                        parameters: vec![],
                    },
                ))
            }
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
