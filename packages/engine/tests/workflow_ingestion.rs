#[cfg(test)]
mod workflow_ingestion_tests {
    use std::fs;

    use engine::workflow::loader::read_workflow_toml;

    #[test]
    fn parses_valid_workflow() {
        let path = format!("{}/tests/fixtures/valid.toml", env!("CARGO_MANIFEST_DIR"));
        let contents = fs::read_to_string(&path).expect("Failed to read TOML file");
        let parsed_toml = read_workflow_toml(contents.as_str()).expect("Failed to parse TOML");

        assert_eq!(parsed_toml.name, "build-test-deploy");
        assert_eq!(parsed_toml.version, 1);
        assert_eq!(parsed_toml.steps.len(), 3);
    }
}
