pub mod parser;


#[macro_export]
macro_rules! string {
    ($str: expr) => {
        String::from($str)
    };
}

#[macro_export]
macro_rules! yaml_string {
    ($str: expr) => {
        Yaml::String(string!($str))
    };
}