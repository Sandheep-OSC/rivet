use engine::workflow::loader::read_workflow_toml;

fn main() {
    let abc = read_workflow_toml("abc=/\"qjd / \"\n");
    println!("{:?}", abc);
}
