use std::io::process::{Command,ProcessOutput};
use std::os;
use std::str::StrExt;

fn main() {
    println!("Hello, my name is Legato, a C++ build tool.");

    let mut compiler = Command::new("g++");

    let current_working_directory = os::getcwd().unwrap();

    compiler.arg("main.cxx");

    match compiler.output(){
        Err(why) => panic!("Could not spawn compiler: {}", why.desc),
        Ok(ProcessOutput {error: err, output: out, status: exit}) => {


        if exit.success() {
                // `out` has type `Vec<u8>`, convert it to a UTF-8 `$str`
                let s = String::from_utf8_lossy(out.as_slice());

                print!("g++ succeeded and stdout was:\n{}", s);
            } else {
                // `err` also has type `Vec<u8>`
                let s = String::from_utf8_lossy(err.as_slice());

                print!("g++ failed and stderr was:\n{}", s);
            }

        },
    }
}
