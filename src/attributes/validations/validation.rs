use crate::{attributes::attribute::Attribute, error::Error};

pub trait Validation {
    fn partially_validate(&mut self, attributes: &[Attribute]) -> Result<(), Error>;
    fn complete(&mut self) -> Result<(), Error>;
}
