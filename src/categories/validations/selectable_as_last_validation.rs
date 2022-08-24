use std::cell::RefCell;
use std::rc::Rc;

use crate::categories::in_memory_category::InMemoryCategory;
use crate::categories::validations::validation::Validation;
use crate::error::{Error, ErrorKind};

/// Validates if the each end category is set as selectable as last by itself or by its parents.
pub struct SelectableAsLastValidation {}

impl SelectableAsLastValidation {
    pub fn new() -> SelectableAsLastValidation {
        SelectableAsLastValidation {}
    }

    fn validate_selectable_as_last(&self, category_pointer: &Rc<RefCell<InMemoryCategory>>) -> Result<(), Error> {
        match category_pointer.try_borrow() {
            Ok(category) => {
                if category.selectable_as_last {
                    return Ok(());
                }

                if category.children.is_empty() {
                    return Err(Error::new(ErrorKind::LastCategoryNotSelectable,
                                          format!("category '{}' has no children and it is not selectable as last",
                                                  category.id).as_str()));
                }

                for child in category.children.as_slice() {
                    match self.validate_selectable_as_last(child) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }

                Ok(())
            }
            Err(error) => Err(Error::new(ErrorKind::FailedToBorrowCategory,
                                         format!("failed to borrow category: {}", error).as_str())),
        }
    }
}

impl Validation for SelectableAsLastValidation {
    fn validate(&self, root_categories: &[Rc<RefCell<InMemoryCategory>>]) -> Result<(), Error> {
        for category in root_categories {
            match self.validate_selectable_as_last(category) {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        Ok(())
    }
}