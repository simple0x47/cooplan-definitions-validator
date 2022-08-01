use std::io::{Error, ErrorKind};

use uuid::Uuid;

use super::attribute::Attribute;

/// Sets a random id to the attribute.
///
/// Error kinds:
///
/// * `InvalidInput`: if the attribute has already an id.
///
/// # Returns
///
/// * `Ok`: returns the ownership of the attribute.
/// * `Error`: contains the error that occurred.
pub fn set_random_id(mut attribute: Attribute) -> Result<Attribute, Error> {
    match attribute.id {
        Some(_) => Err(Error::new(
            ErrorKind::InvalidInput,
            "Attribute already has an id",
        )),
        None => {
            let id = Uuid::new_v4();
            attribute.id = Some(id.to_string());

            Ok(attribute)
        }
    }
}
