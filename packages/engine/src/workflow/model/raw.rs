use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RawWorkflow {
    pub name: String,
    pub version: u32,
    pub steps: HashMap<String, RawStep>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawStep {
    pub run: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RawRetry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<RawApproval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<RawRollback>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawRetry {
    pub max_attempts: u32,
    pub on: Vec<RetryCondition>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryCondition {
    InfraError,
    Timeout,
    CommandFailure,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawApproval {
    pub required: bool,
    pub reviewers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawRollback {
    pub run: Vec<String>,
}
