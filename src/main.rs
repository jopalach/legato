
extern crate legato;

use std::os;
use legato::compiler::{Compiler};

fn main() {
    println!("Hello, my name is Legato, a C++ build tool.");

    let mut compiler = Compiler::new("g++");
    compiler.arg("main.cxx");

    compiler.compile();

    let current_working_directory = os::getcwd().unwrap();
}
