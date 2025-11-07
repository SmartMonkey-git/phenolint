use crate::error::FixingError;

pub struct Patcher;

impl Patcher {
    pub fn patch(&self) -> Result<String, FixingError> {
        Ok("".to_string())
    }
}
