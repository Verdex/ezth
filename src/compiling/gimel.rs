
use super::alef::*;

#[derive(Debug)]
pub enum GimelError {

}

impl std::fmt::Display for GimelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            _ => unreachable!(),
        }
    }
}

impl std::error::Error for GimelError { }

pub fn compile() -> Result<Vec<AlefFun>, GimelError> {
    todo!()
}