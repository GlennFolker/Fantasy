use std::env::var;
use embed_resource::compile;

fn main() {
    let target = var("TARGET").unwrap();
    if target.contains("windows") {
        compile("icon.rc");
    }
}
