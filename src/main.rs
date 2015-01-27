
extern crate legato;

//use std::os;
use legato::compiler::gcc;
use legato::compiler::build;

fn main() {
    println!("Hello, my name is Legato, a C++ build tool.");

    let mut compiler = gcc::Compiler::new("g++");
    compiler.arg("-MMD");
    compiler.arg("main.cxx");

    let builder = build::Builder::new(&compiler);

    builder.build();

    //let current_working_directory = os::getcwd().unwrap();
}
