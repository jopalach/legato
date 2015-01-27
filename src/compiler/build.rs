use compiler::common::{HasCompiler};

pub struct Builder<'a>{
    compiler: &'a (HasCompiler + 'a),
}

impl<'a> Builder<'a>{

    pub fn new(compiler: &'a HasCompiler) -> Builder<'a>{
        Builder {
            compiler: compiler,
        }
    }

    pub fn build(&self) {
        self.compiler.compile();
    }


}
