use std::io::process::{ProcessOutput};
use std::io::{IoResult};

pub trait HasCompiler{
    fn compile(&self) -> IoResult<ProcessOutput>;
}
