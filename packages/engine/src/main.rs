use engine::workflow::loader::read_workflow_toml;
use std::fs;

fn main() {
    let content = fs::read_to_string(
        "/home/lnxdev9/zenith/dev/rust/rivet/packages/engine/tests/fixtures/invalid_run.toml",
    )
    .expect("Failed to read TOML file");
    let result = read_workflow_toml(content.as_str());
    println!("Result: {:?}", result);
}
