use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Description(String);

#[derive(Debug, thiserror::Error)]
pub enum DescriptionError {
    #[error("The description cannot be empty")]
    Empty,
    #[error("The description cannot be longer than 500 bytes")]
    TooLong,
}
impl TryFrom<String> for Description {
    type Error = DescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value.to_string()))
    }
}

impl TryFrom<&str> for Description {
    type Error = DescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_string()))
    }
}

impl Description {
    fn validate(title: &str) -> Result<(), DescriptionError> {
        if title.is_empty() {
            Err(DescriptionError::Empty)
        } else if title.len() > 50 {
            Err(DescriptionError::TooLong)
        } else {
            Ok(())
        }
    }
}
