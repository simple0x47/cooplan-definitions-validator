use uuid::Uuid;

use crate::categories::source_category::SourceCategory;
use crate::error::{Error, ErrorKind};

/// Sets a random id to the category.
///
/// Error kinds:
///
/// * `CannotOverrideId`: if the category has already an id.
///
/// # Returns
///
/// * `Ok`: returns the ownership of the category.
/// * `Err`: contains the error that occurred.
pub fn set_random_id(source_category: &mut SourceCategory) -> Result<(), Error> {
    match source_category.id {
        Some(_) => Err(Error::new(
            ErrorKind::CannotOverrideId,
            format!("category '{}' already has an id", source_category.name).as_str(),
        )),
        None => {
            let id = Uuid::new_v4();
            source_category.id = Some(id.to_string());

            Ok(())
        }
    }
}
