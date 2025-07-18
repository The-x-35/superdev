use std::fmt;

#[derive(Debug)]
pub struct SerializeError;

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Serialization error")
    }
}

impl std::error::Error for SerializeError {}

pub trait Serialize {
	fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
	fn deserialize(base: &[u8]) -> Result<Self, SerializeError>;
}
