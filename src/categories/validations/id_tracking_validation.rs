use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::category_id_tracker::{CategoryEntry, CategoryIdTracker};
use crate::categories::category_id_tracker_file_io::CategoryIdTrackerFileIO;
use crate::categories::category_id_tracker_io::CategoryIdTrackerIO;
use crate::categories::validations::validation::Validation;
use crate::error::{Error, ErrorKind};

pub struct IdTrackingValidation {}

impl IdTrackingValidation {
    pub fn new() -> IdTrackingValidation {
        IdTrackingValidation {}
    }

    fn track_category(
        &self,
        category_pointer: &Rc<RefCell<Category>>,
        category_id_tracker: &mut CategoryIdTracker,
    ) -> Result<(), Error> {
        match category_pointer.try_borrow() {
            Ok(category) => {
                match category_id_tracker.track_category(category.id.clone().as_str()) {
                    Ok(_) => {
                        for child_pointer in category.children.as_slice() {
                            match self.track_category(child_pointer, category_id_tracker) {
                                Ok(_) => (),
                                Err(error) => return Err(error),
                            }
                        }

                        Ok(())
                    }
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(Error::new(
                ErrorKind::FailedToBorrowCategory,
                format!("failed to borrow category: {}", error).as_str(),
            )),
        }
    }
}

impl Validation for IdTrackingValidation {
    fn validate(&self, root_categories: &[Rc<RefCell<Category>>]) -> Result<(), Error> {
        let category_id_tracker_io: Box<dyn CategoryIdTrackerIO> =
            Box::new(CategoryIdTrackerFileIO::new());

        match category_id_tracker_io.read_entries() {
            Ok(category_entries) => {
                let mut category_id_tracker = CategoryIdTracker::new(category_entries);

                for category_pointer in root_categories {
                    match self.track_category(category_pointer, &mut category_id_tracker) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }

                match category_id_tracker.close() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(Error::new(
                ErrorKind::FailedToReadCategory,
                format!("failed to read categories: {}", error).as_str(),
            )),
        }
    }
}
