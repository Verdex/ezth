
#[derive(Debug)]
pub enum RuntimeError {
    Type { src: &'static str, expected: &'static str },
    Words(&'static str),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            _ => todo!(),
        }
    }
}

impl std::error::Error for RuntimeError { }