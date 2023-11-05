use std::str::FromStr;

use app::util::{IdGenerator, ParseError, ParseResult};
use ulid::Ulid;

#[derive(Clone)]
pub struct MockUlidGenerator {
    value: Ulid,
}

impl MockUlidGenerator {
    pub fn with_fixed_value(value: &str) -> Self {
        MockUlidGenerator {
            value: Ulid::from_string(value).unwrap(),
        }
    }
}

impl IdGenerator<Ulid> for MockUlidGenerator {
    fn generate(&self) -> Ulid {
        self.value
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
