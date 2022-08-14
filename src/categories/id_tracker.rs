use std::collections::HashMap;

use crate::error::{Error, ErrorKind};

use super::category::Category;

pub struct Entry {
    pub id: String,
}

pub struct IdTracker {
    entries: HashMap<String, Entry>,
    found_entries: HashMap<String, Entry>,
}

impl IdTracker {
    pub fn new(entries: HashMap<String, Entry>) -> IdTracker {
        IdTracker {
            entries,
            found_entries: HashMap::new(),
        }
    }

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
                                format!("Duplicated category id {}", id),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::IdNotFound,
                            format!("Category id {} does not exist", id),
                        ))
                    }
                }
            }
            None => Err(Error::new(
                ErrorKind::MissingId,
                "Category has no id".to_string(),
            )),
        }
    }

    fn track_parent(&mut self, parent: &Option<String>) -> Result<(), Error> {
        match parent {
            Some(parent) => {
                if !self.found_entries.contains_key(parent) && !self.entries.contains_key(parent) {
                    return Err(Error::new(
                        ErrorKind::ParentNotFound,
                        "Parent category not found".to_string(),
                    ));
                }

                Ok(())
            }
            None => Ok(()),
        }
    }

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
            format!("Some category ids were not tracked: {}", missing_ids),
        ));
    }
}
