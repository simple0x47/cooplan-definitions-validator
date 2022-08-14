use crate::error::{Error, ErrorKind};

use uuid::Uuid;

use crate::attributes::attribute::Attribute;

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
            "Attribute already has an id".to_string(),
        )),
        None => {
            let id = Uuid::new_v4();
            attribute.id = Some(id.to_string());

            Ok(attribute)
        }
    }
}
