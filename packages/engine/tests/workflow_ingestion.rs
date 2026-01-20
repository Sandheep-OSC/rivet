#[cfg(test)]
mod workflow_ingestion_tests {
    use std::fs;

    use engine::workflow::{loader::read_workflow_toml, model::error::WorkflowError};

    #[test]
    fn parses_valid_workflow() {
        let path = format!("{}/tests/fixtures/valid.toml", env!("CARGO_MANIFEST_DIR"));
        let contents = fs::read_to_string(&path).expect("Failed to read TOML file");
        let parsed_toml = read_workflow_toml(contents.as_str()).expect("Failed to parse TOML");

        assert_eq!(parsed_toml.name, "build-test-deploy");
        assert_eq!(parsed_toml.version, 1);
        assert_eq!(parsed_toml.steps.len(), 3);
    }

    #[test]
    fn bad_retry_workflow() {
        let path = format!(
            "{}/tests/fixtures/bad_retry.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = fs::read_to_string(&path).expect("Failed to read TOML file");

        let result = read_workflow_toml(content.as_str());
        assert!(matches!(result, Err(WorkflowError::TomlParseError(_))));
    }

    #[test]
    fn invalid_run_workflow() {
        let path = format!(
            "{}/tests/fixtures/invalid_run.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = fs::read_to_string(&path).expect("Failed to load TOML");
        let result = read_workflow_toml(content.as_str());

        assert!(matches!(result, Err(WorkflowError::TomlParseError(_))));
    }

    #[test]
    fn missing_name_workflow() {
        let path = format!(
            "{}/tests/fixtures/missing_name.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = fs::read_to_string(&path).expect("");
        let result = read_workflow_toml(content.as_str());

        assert!(matches!(result, Err(WorkflowError::TomlParseError(_))))
    }

    #[test]
    fn unknown_field_workflow() {
        let path = format!(
            "{}/tests/fixtures/unknown_field.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let content = fs::read_to_string(&path).expect("");
        let result = read_workflow_toml(content.as_str());

        println!("{:?}", result);

        assert!(matches!(result, Err(WorkflowError::TomlParseError(_))));
    }
}
