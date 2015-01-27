
extern crate legato;

use std::os;
use legato::compiler::gcc;
use legato::compiler::build;

fn main() {
    println!("Hello, my name is Legato, a C++ build tool.");

    let mut compiler = gcc::Compiler::new("g++");
    compiler.arg("-MMD");
    compiler.arg("main.cxx");

    let builder = match os::getcwd().unwrap().as_str()
    {
        Some(path) => build::Builder::new(path),
        None =>  unreachable!(),
    };
    builder.build();
}
