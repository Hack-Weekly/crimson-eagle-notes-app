use thiserror::Error;

#[derive(Debug, Error)]
pub enum NotesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Internal(String),
}

impl serde::Serialize for NotesError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
