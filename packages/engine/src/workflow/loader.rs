use crate::workflow::{
    model::{error::WorkflowError, raw::RawWorkflow},
    validators::parser::toml::validate_workflow_json,
};

pub fn read_workflow_toml(raw_toml_input: &str) -> Result<RawWorkflow, WorkflowError> {
    validate_workflow_json(raw_toml_input)
}
