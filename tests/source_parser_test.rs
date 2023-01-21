#[cfg(test)]
mod source_parser_test {
    use std::fs;
    use std::path::PathBuf;

    use code_nix::constants::MANIFEST_DIR;
    use code_nix::source::{ create_parser, API, HttpMethod, SourceParser, ParserType, MINE };

    #[ctor::ctor]
    fn setup() {
        println!("workdir is {}", MANIFEST_DIR);
    }

    #[test]
    fn get_full_context_from_str() {
        let mut path = PathBuf::from(MANIFEST_DIR);
        path.push("tests/samples/full_swagger.io.yaml");

        let yaml = fs::read_to_string(path.as_path())
            .expect("read file failed");

        let ysp = create_parser(ParserType::YamlParser);
        let ctx = (*ysp).get_context_from_str(yaml.as_str());

        assert_eq!(ctx.base_path, String::from("/v1"));

        assert_eq!(ctx.api_list, vec![
            API {
                url: String::from("/users"),
                methods: vec![
                    (String::from("get"), HttpMethod {
                        summary: String::from("Returns a list of users."),
                        produces: vec![MINE(String::from("application/json"))]
                    })
                ]
            },
            API {
                url: String::from("/user"),
                methods: vec![
                    (String::from("post"), HttpMethod {
                        summary: String::from("Post a new user"),
                        produces: vec![]
                    })
                ]
            },
            API {
                url: String::from("/users/{userId}"),
                methods: vec![
                    (String::from("get"), HttpMethod {
                        summary: String::from("Returns a user"),
                        produces: vec![]
                    }),
                    (String::from("patch"), HttpMethod {
                        summary: String::from("Patch a user"),
                        produces: vec![]
                    }),
                    (String::from("delete"), HttpMethod {
                        summary: String::from("Remove a user"),
                        produces: vec![]
                    }),
                ]
            },
            API {
                url: String::from("/teachers"),
                methods: vec![
                    (String::from("get"), HttpMethod {
                        summary: String::from("Returns a list of teacher."),
                        produces: vec![]
                    })
                ]
            }
        ]);

        assert_eq!(ctx.model_list, vec![]);
    }
}