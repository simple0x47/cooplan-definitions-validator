use crate::categories::category::InMemoryCategory;
use crate::error::Error;

pub trait Validation {
    fn validate(&self, categories: &[InMemoryCategory]) -> Result<(), Error>;
}