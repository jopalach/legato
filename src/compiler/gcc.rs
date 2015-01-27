use std::io::process::{Command, ProcessOutput};
use std::io::{IoResult};
use std::str::StrExt;
use compiler::common::{HasCompiler};

pub struct Compiler {
    exe: String,
    args: Vec<String>,
}

impl Compiler{
    pub fn new(exe: &str) -> Compiler {
        Compiler {
            exe: exe.to_string(),
            args: Vec::new(),
        }
    }

    pub fn arg(&mut self, arg: &str) {
        self.args.push(arg.to_string())
    }
}

impl HasCompiler for Compiler{
    fn compile(&self) -> IoResult<ProcessOutput> {
        let mut compiler = Command::new(self.exe.as_slice());

        for arg in self.args.iter(){
            compiler.arg(arg.as_slice());
        }

        compiler.output()
    }
}
