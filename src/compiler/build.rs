
pub struct Builder{
    currentWorkingDirectory: String,
}

impl Builder{

    pub fn new(currentWorkingDirectory: &str) -> Builder{
        Builder {
            currentWorkingDirectory: currentWorkingDirectory.to_string(),
        }
    }

    pub fn build(&self){
        println!("Building in {}", self.currentWorkingDirectory);

    }
}
