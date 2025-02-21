use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Title(String);

#[derive(Debug, thiserror::Error)]
pub enum TitleError {
    #[error("The title cannot be empty")]
    Empty,
    #[error("The title cannot be longer than 50 bytes")]
    TooLong,
}

impl TryFrom<String> for Title {
    type Error = TitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for Title {
    type Error = TitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value)?;
        Ok(Self(value.to_string()))
    }
}

impl Title {
    fn validate(title: &str) -> Result<(), TitleError> {
        if title.is_empty() {
            Err(TitleError::Empty)
        } else if title.len() > 50 {
            Err(TitleError::TooLong)
        } else {
            Ok(())
        }
    }
}
