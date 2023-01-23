use std::{fmt::{Debug}, error::Error};

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
pub struct HttpMethod(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct API {
    pub url: String,
    pub method: HttpMethod,
    pub summary: String,
    pub produces: Vec<MINE>,
    // TODO: 
    // pub consumes: Vec<MINE>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Metadata {
    pub base_path: String,
    pub api_list: Vec<API>,
    pub model_list: Vec<Model>,
}

pub trait MetadataParser: PartialEq + Eq {
    fn from_str(&self, str: &str) -> Result<Metadata, Box<dyn Error>>;
}