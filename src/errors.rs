use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("IO")] 
    IO(#[from] std::io::Error),
}
