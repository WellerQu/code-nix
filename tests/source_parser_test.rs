#[cfg(test)]
mod source_parser_test {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use code_nix::constants::MANIFEST_DIR;
    use code_nix::source::{ create_parser, API, HttpMethod, MINE, SourceParser, ParserType };

    #[ctor::ctor]
    fn setup() {
        println!("workdir is {}", MANIFEST_DIR);
    }

    #[test]
    fn get_context_from_str() {
        let mut path = PathBuf::from(MANIFEST_DIR);
        path.push("tests/samples/swagger.io.yaml");

        let yaml = fs::read_to_string(path.as_path())
            .expect("read file failed");

        let ysp = create_parser(ParserType::YamlParser);
        let ctx = (*ysp).get_context_from_str(yaml.as_str());

        assert_eq!(ctx.base_path, String::from("/v1"));

        assert_eq!(ctx.api_list, vec![
            API {
                url: String::from("/users"),
                methods: HashMap::from([
                    (String::from("get"), HttpMethod {
                        produces: vec![MINE(String::from("application/json"))]
                    })
                ])
            },
            API {
                url: String::from("/teachers"),
                methods: HashMap::from([
                    (String::from("get"), HttpMethod {
                        produces: vec![MINE(String::from("application/json"))]
                    })
                ])
            }
        ]);

        assert_eq!(ctx.model_list, vec![]);
    }
}