use crate::error::{Error, ErrorKind};

use uuid::Uuid;

use crate::categories::category::Category;

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
pub fn set_random_id(mut category: Category) -> Result<Category, Error> {
    match category.id {
        Some(_) => Err(Error::new(
            ErrorKind::CannotOverrideId,
            "Category already has an id".to_string(),
        )),
        None => {
            let id = Uuid::new_v4();
            category.id = Some(id.to_string());

            Ok(category)
        }
    }
}
