use std::io::process::{Command,ProcessOutput};
use std::str::StrExt;

pub struct Compiler {
    exe: String,
    args: Vec<String>,
}

impl Compiler{
    pub fn arg(&mut self, arg: &str) {
        self.args.push(arg.to_string())
    }

    pub fn new(exe: &str) -> Compiler {
        Compiler {
            exe: exe.to_string(),
            args: Vec::new(),
        }
    }

    pub fn compile(&self) {
        let mut compiler = Command::new(self.exe.as_slice());

        for arg in self.args.iter(){
            compiler.arg(arg.as_slice());
        }

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
}
