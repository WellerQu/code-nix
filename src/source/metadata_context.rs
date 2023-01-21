use std::{fmt::Debug, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct  Field {
    pub field_name: String,
    pub field_type: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Model {
    pub fields: Vec<Field>
}

#[derive(Debug, PartialEq, Eq)]
pub struct MINE(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct HttpMethod {
    pub summary: String,
    pub produces: Vec<MINE>
}

#[derive(Debug, PartialEq, Eq)]
pub struct API {
    pub url: String,
    pub methods: HashMap<String, HttpMethod>
}

#[derive(Debug, PartialEq, Eq)]
pub struct MetadataContext {
    pub base_path: String,
    pub api_list: Vec<API>,
    pub model_list: Vec<Model>,
}

pub trait SourceParser: PartialEq + Eq {
    fn get_context_from_str(&self, str: &str) -> MetadataContext;
}