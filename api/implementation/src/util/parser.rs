use chrono::{DateTime, Utc};
use ulid::Ulid;

pub enum ParseError {
    InvalidUlid(String),
    InvalidDatetime(String),
}

type ParseResult<T> = Result<T, ParseError>;

pub fn validate_datetime_from_string(datetime: String) -> ParseResult<DateTime<Utc>> {
    if let Ok(datatime) = &datetime.parse::<DateTime<Utc>>() {
        return Ok(*datatime);
    }

    Err(ParseError::InvalidDatetime(datetime))
}
