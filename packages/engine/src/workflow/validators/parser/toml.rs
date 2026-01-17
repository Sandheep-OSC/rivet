use serde_json::Value;

use crate::workflow::model::{error::WorkflowError, raw::RawWorkflow};
use workflow_contracts::workflow_export::WORKFLOW_SCHEMA_V1;

fn format_field_path(instance_path: &str) -> String {
    match instance_path {
        "" => "workflow".to_string(),
        "/name" => "workflow name".to_string(),
        "/version" => "workflow version".to_string(),
        path if path.starts_with("/steps/") => {
            let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
            match parts.as_slice() {
                ["steps", step_name] => format!("step '{}'", step_name),
                ["steps", step_name, "run"] => format!("step '{}' run commands", step_name),
                ["steps", step_name, "depends_on"] => format!("step '{}' dependencies", step_name),
                ["steps", step_name, "retry", field] => {
                    format!("step '{}' retry {}", step_name, field)
                }
                ["steps", step_name, "approval", field] => {
                    format!("step '{}' approval {}", step_name, field)
                }
                ["steps", step_name, "rollback", field] => {
                    format!("step '{}' rollback {}", step_name, field)
                }
                _ => format!("step '{}'", parts.get(1).unwrap_or(&"unknown")),
            }
        }
        _ => instance_path.trim_start_matches('/').replace('/', "."),
    }
}

fn format_error_message(error: &jsonschema::ValidationError) -> String {
    let location = format_field_path(error.instance_path().as_str());
    let message = error.to_string();

    format!("{}: {}", location, message)
}

pub fn validate_workflow_json(toml_value: &str) -> Result<RawWorkflow, WorkflowError> {
    let schema: Value = serde_json::from_str(WORKFLOW_SCHEMA_V1)
        .map_err(|err| WorkflowError::TomlParseError(format!("Invalid schema: {}", err)))?;

    let workflow: RawWorkflow =
        toml::from_str(toml_value).map_err(|err| WorkflowError::TomlParseError(err.to_string()))?;

    let instance: Value = serde_json::to_value(&workflow).map_err(|err| {
        WorkflowError::ValidationError(format!("Failed to convert workflow: {}", err))
    })?;

    if jsonschema::is_valid(&schema, &instance) {
        Ok(workflow)
    } else {
        match jsonschema::validate(&schema, &instance) {
            Ok(_) => Ok(workflow),
            Err(error) => {
                let message = format_error_message(&error);
                Err(WorkflowError::ValidationError(message))
            }
        }
    }
}
