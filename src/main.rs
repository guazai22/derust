mod derust_to_rust;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::process::Command;

fn main() {
    let rust_content: String =
        derust_to_rust::derust_to_rust(fs::read_to_string("./rust_app/src/main.drs").unwrap());
    let rust_file = String::from("./rust_app/src/main.rs");
    fs::write(&rust_file, &rust_content).unwrap();
    Command::new("rustfmt").arg(&rust_file).status().expect("");
    println!("\nFORMATED RUST FILE:");
    Command::new("cat").arg(&rust_file).status().expect("");
}
