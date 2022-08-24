use std::collections::HashMap;

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
    /// * `MissingId` - if the category has no id.
    /// * `IdNotFound` - if the category's id could not be found.
    /// * `DuplicatedId` - if the category's id is duplicated.
    /// * `ParentNotFound` if the category's parent could not be found.
    pub fn track_category(&mut self, category: &Category) -> Result<(), Error> {
        match &category.id {
            Some(id) => {
                let entry = self.entries.remove(id);

                match entry {
                    Some(entry) => {
                        self.found_entries.insert(id.to_string(), entry);

                        let parent_tracking_result = self.track_parent(&category.parent);

                        if parent_tracking_result.is_err() {
                            return parent_tracking_result;
                        }

                        Ok(())
                    }
                    None => {
                        if self.found_entries.contains_key(id) {
                            return Err(Error::new(
                                ErrorKind::DuplicatedId,
                                format!("Duplicated category id {}", id).as_str(),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::IdNotFound,
                            format!("Category id {} does not exist", id).as_str(),
                        ))
                    }
                }
            }
            None => Err(Error::new(
                ErrorKind::MissingId,
                "Category has no id",
            )),
        }
    }

    fn track_parent(&mut self, parent: &Option<String>) -> Result<(), Error> {
        match parent {
            Some(parent) => {
                if !self.found_entries.contains_key(parent) && !self.entries.contains_key(parent) {
                    return Err(Error::new(
                        ErrorKind::ParentNotFound,
                        "Parent category not found",
                    ));
                }

                Ok(())
            }
            None => Ok(()),
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
