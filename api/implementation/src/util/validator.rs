use ulid::Ulid;

pub enum ValidationError {
    InvalidUlid(String),
}

type ValidationResult<T> = Result<T, ValidationError>;

pub fn validate_ulid(id: String) -> ValidationResult<Ulid> {
    if let Ok(ulid) = Ulid::from_string(&id) {
        return Ok(ulid);
    }

    return Err(ValidationError::InvalidUlid(id));
}
