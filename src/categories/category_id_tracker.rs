use std::cell::RefCell;
use std::collections::HashMap;

use crate::categories::category::Category;
use crate::error::{Error, ErrorKind};

use super::source_category::SourceCategory;

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
    pub fn track_category(&mut self, id: &str) -> Result<(), Error> {
        let entry = self.entries.remove(id);

        match entry {
            Some(entry) => {
                self.found_entries.insert(id.to_string(), entry);

                Ok(())
            }
            None => {
                if self.found_entries.contains_key(id) {
                    return Err(Error::new(
                        ErrorKind::DuplicatedId,
                        format!("duplicated category id {}", id).as_str(),
                    ));
                }

                Err(Error::new(
                    ErrorKind::IdNotFound,
                    format!("category id {} does not exist", id).as_str(),
                ))
            }
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
