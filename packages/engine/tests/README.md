# Engine Tests

This directory contains integration tests for the workflow engine, focusing on TOML parsing and validation.

## Test Cases

### Existing Tests

- **parses_valid_workflow**: Verifies successful parsing of a well-formed workflow TOML
- **bad_retry_workflow**: Tests failure for invalid retry configuration (e.g., unsupported "on" values causing TomlParseError)
- **invalid_run_workflow**: Tests failure when `run` field is a string instead of array (TomlParseError)
- **missing_name_workflow**: Tests failure for workflows missing the required `name` field (TomlParseError)
- **unknown_field_workflow**: Tests failure for workflows with extra/unknown fields at root level (TomlParseError due to deny_unknown_fields)

### Recommended Additional Tests

#### Schema Validation Errors (ValidationError)

These test cases where TOML deserializes successfully but violates JSON schema rules:

- **invalid_version_workflow**: `version = 0` (violates minimum 1)
- **empty_steps_workflow**: Empty `steps = {}` (violates minProperties 1)
- **empty_run_workflow**: `run = []` (violates minItems 1)
- **invalid_retry_max_attempts**: `max_attempts = 0` (violates minimum 1) with valid `on` values
- **invalid_approval_reviewers**: `reviewers = []` (violates minItems 1)
- **invalid_rollback_run**: `run = []` in rollback (violates minItems 1)

#### Type Validation Errors (TomlParseError)

Cases where serde deserialization fails due to type mismatches:

- **non_string_run_workflow**: `run = [1, 2, 3]` (expects array of strings)
- **non_integer_version_workflow**: `version = "one"` (expects u32)
- **non_array_depends_on_workflow**: `depends_on = "build"` (expects array)
- **non_boolean_approval_required**: `required = "yes"` (expects bool)

#### Structural Errors

- **duplicate_step_names**: Two steps with the same name (tests HashMap behavior)
- **circular_dependency**: Step depending on itself (may require post-parse validation)

## Running Tests

```bash
cargo test -p engine
```

## Fixtures

Test fixtures are located in the `fixtures/` directory. Each invalid test case should have a corresponding `.toml` file demonstrating the specific validation failure.

