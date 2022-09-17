use cooplan_definitions_lib::source_attribute::SourceAttribute;
use uuid::Uuid;

use crate::error::{Error, ErrorKind};

/// Sets a random id to the attribute.
///
/// Error kinds:
///
/// * `CannotOverrideId`: if the attribute has already an id.
///
/// # Returns
///
/// * `Ok`: id has been generated correctly.
/// * `Error`: contains the error that occurred.
pub fn set_random_id(attribute: &mut SourceAttribute) -> Result<(), Error> {
    match attribute.id {
        Some(_) => Err(Error::new(
            ErrorKind::CannotOverrideId,
            "attribute already has an id",
        )),
        None => {
            let id = Uuid::new_v4();
            attribute.id = Some(id.to_string());

            Ok(())
        }
    }
}
