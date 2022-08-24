use uuid::Uuid;

use crate::attributes::attribute::Attribute;
use crate::error::{Error, ErrorKind};

/// Sets a random id to the attribute.
///
/// Error kinds:
///
/// * `CannotOverrideId`: if the attribute has already an id.
///
/// # Returns
///
/// * `Ok`: returns the ownership of the attribute.
/// * `Error`: contains the error that occurred.
pub fn set_random_id(mut attribute: Attribute) -> Result<Attribute, Error> {
    match attribute.id {
        Some(_) => Err(Error::new(
            ErrorKind::CannotOverrideId,
            "Attribute already has an id",
        )),
        None => {
            let id = Uuid::new_v4();
            attribute.id = Some(id.to_string());

            Ok(attribute)
        }
    }
}
