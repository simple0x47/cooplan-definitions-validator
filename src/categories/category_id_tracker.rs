use std::cell::RefCell;
use std::collections::HashMap;

use crate::categories::in_memory_category::InMemoryCategory;
use crate::error::{Error, ErrorKind};

use super::category::Category;

pub struct CategoryEntry {
    pub id: String,
}

pub struct CategoryIdTracker {
    entries: HashMap<String, CategoryEntry>,
    found_entries: HashMap<String, CategoryEntry>,
}

impl CategoryIdTracker {
    pub fn new(entries: HashMap<String, CategoryEntry>) -> CategoryIdTracker {
        CategoryIdTracker {
            entries,
            found_entries: HashMap::new(),
        }
    }

    /// Tracks the id and the parent of the category.
    ///
    /// Error kinds:
    ///
    /// * `IdNotFound` - if the category's id could not be found.
    /// * `DuplicatedId` - if the category's id is duplicated.
    pub fn track_category(&mut self, category_pointer: &RefCell<InMemoryCategory>) -> Result<(), Error> {
        match category_pointer.try_borrow() {
            Ok(category) => {
                let entry = self.entries.remove(category.id.clone().as_str());

                match entry {
                    Some(entry) => {
                        self.found_entries.insert(category.id.clone(), entry);

                        Ok(())
                    }
                    None => {
                        if self.found_entries.contains_key(category.id.clone().as_str()) {
                            return Err(Error::new(
                                ErrorKind::DuplicatedId,
                                format!("duplicated category id {}", category.id).as_str(),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::IdNotFound,
                            format!("category id {} does not exist", category.id).as_str(),
                        ))
                    }
                }
            }
            Err(error) => Err(Error::new(ErrorKind::FailedToBorrowCategory,
                                         format!("failed to borrow category: {}", error).as_str())),
        }
    }

    /// To be called after all categories have been tracked.
    ///
    /// Error kinds:
    ///
    /// * `IdNotTracked` - if there are category ids that have not been tracked.
    pub fn close(&self) -> Result<(), Error> {
        if self.entries.is_empty() {
            return Ok(());
        }

        let mut missing_ids: String = "".to_string();

        for (id, _) in self.entries.iter() {
            missing_ids.push_str(&format!("{}, ", id));
        }

        missing_ids = missing_ids.trim_end_matches(", ").to_string();

        return Err(Error::new(
            ErrorKind::IdNotTracked,
            format!("Some category ids were not tracked: {}", missing_ids).as_str(),
        ));
    }
}
