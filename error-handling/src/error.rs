use failure_derive::*;

#[derive(Debug, Fail)]
pub enum TransationError {
    #[fail(display="Could not load file : {}",0)]
    LoaderError(std::io::Error),
    #[fail(display="Could not parse file : {}",0)]
    ParseError(serde_json::Error),
    #[fail(display="Error : {}",0)]
    Mess(& 'static str),
}

impl From<std::io::Error> for TransationError {
    fn from(e: std::io::Error) -> Self {
        TransationError::LoaderError(e)
    }
}
impl From<serde_json::Error> for TransationError {
    fn from(e: serde_json::Error) -> Self {
        TransationError::ParseError(e)
    }
}
impl From<& 'static str> for TransationError {
    fn from(e: & 'static str) -> Self {
        TransationError::Mess(e)
    }
}