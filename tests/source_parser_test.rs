#[cfg(test)]
mod source_parser_test {
    use std::fs;
    use std::path::PathBuf;

    use code_nix::constants::MANIFEST_DIR;
    use code_nix::source::{create_parser, HttpMethod, MetadataParser, ParserType, API, MINE};

    #[ctor::ctor]
    fn setup() {
        println!("workdir is {}", MANIFEST_DIR);
    }

    #[test]
    fn open_api_meta() {
        let mut path = PathBuf::from(MANIFEST_DIR);
        path.push("tests/samples/full_swagger.io.yaml");

        let yaml = fs::read_to_string(path.as_path()).expect("read file failed");

        let ysp = create_parser(ParserType::YamlParser);
        let meta = (*ysp).from_str(yaml.as_str()).expect("parse doc failed");

        assert_eq!(meta.base_path, String::from("/v1"));

        assert_eq!(
            meta.api_list,
            vec![
                API {
                    url: String::from("/users"),
                    method: HttpMethod(String::from("get")),
                    summary: String::from("Returns a list of users."),
                    produces: vec![MINE(String::from("application/json"))]
                },
                API {
                    url: String::from("/user"),
                    method: HttpMethod(String::from("post")),
                    summary: String::from("Post a new user"),
                    produces: vec![]
                },
                API {
                    url: String::from("/users/{userId}"),
                    method: HttpMethod(String::from("get")),
                    summary: String::from("Returns a user"),
                    produces: vec![],
                },
                API {
                    url: String::from("/users/{userId}"),
                    method: HttpMethod(String::from("patch")),
                    summary: String::from("Patch a user"),
                    produces: vec![],
                },
                API {
                    url: String::from("/users/{userId}"),
                    method: HttpMethod(String::from("delete")),
                    summary: String::from("Remove a user"),
                    produces: vec![],
                },
                API {
                    url: String::from("/teachers"),
                    method: HttpMethod(String::from("get")),
                    summary: String::from("Returns a list of teacher."),
                    produces: vec![]
                }
            ]
        );

        assert_eq!(meta.model_list, vec![]);
    }
}
