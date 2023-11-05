use std::{fmt::Display, str::FromStr};

use ulid::Ulid;

pub enum ParseError {
    InvalidUlid(String),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait IdGenerator<T>: Send + Sync + 'static {
    fn generate(&self) -> T;
    fn parse(&self, id: &str) -> ParseResult<T>;
    fn to_string(&self, id: &T) -> String;
}

#[derive(Clone)]
pub struct UlidGenerator;

impl UlidGenerator {
    pub fn new() -> Self {
        UlidGenerator {}
    }
}

impl Default for UlidGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator<Ulid> for UlidGenerator {
    fn generate(&self) -> Ulid {
        Ulid::new()
    }

    fn parse(&self, id: &str) -> ParseResult<Ulid> {
        if let Ok(ulid) = Ulid::from_str(id) {
            return Ok(ulid);
        }

        Err(ParseError::InvalidUlid(id.to_string()))
    }

    fn to_string(&self, id: &Ulid) -> String {
        id.to_string()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidUlid(id) => write!(f, "Unable to parse {} to Ulid", id),
        }
    }
}
