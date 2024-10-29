use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid tag")]
    InvalidTag,
    #[error("No embedded data")]
    NoEmbeddedData,
}
