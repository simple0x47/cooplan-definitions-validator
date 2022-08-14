use crate::error::{Error, ErrorKind};
use std::collections::HashMap;

use crate::attributes::attribute::Attribute;

pub struct Entry {
    pub id: String,
    pub attribute_type: String,
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

    /// Tracks the id of the attribute.
    ///
    /// Error kinds:
    ///
    /// * `MissingId` - if the attribute has no id.
    /// * `IdNotFound`- if the attribute's id could not be found.
    /// * `DuplicatedId` - if the attribute's id is duplicated.
    /// * `TypeChanged` - if the attribute's type has been changed.
    ///
    /// # Arguments
    ///
    /// * `attribute` - Attribute whose id is going to be tracked.
    pub fn track_attribute(&mut self, attribute: &Attribute) -> Result<(), Error> {
        match &attribute.id {
            Some(id) => {
                let entry = self.entries.remove(id);

                match entry {
                    Some(entry) => {
                        if entry.attribute_type != attribute.data_type {
                            return Err(Error::new(
                                ErrorKind::TypeChanged,
                                format!(
                                    "Expected attribute with type {} but found {}",
                                    entry.attribute_type, attribute.data_type
                                ),
                            ));
                        }

                        self.found_entries.insert(id.to_string(), entry);

                        Ok(())
                    }
                    None => {
                        if self.found_entries.contains_key(id) {
                            return Err(Error::new(
                                ErrorKind::DuplicatedId,
                                format!("Duplicated attribute id {}", id),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::IdNotFound,
                            format!("Attribute with id {} does not exist", id),
                        ))
                    }
                }
            }
            None => Err(Error::new(
                ErrorKind::MissingId,
                "Attribute has no id".to_string(),
            )),
        }
    }

    /// To be called after all attributes have been tracked.
    ///
    /// Error kinds:
    ///
    /// * `IdNotTracked` - if there are attribute ids that have not been tracked.
    pub fn close(&self) -> Result<(), Error> {
        if self.entries.is_empty() {
            return Ok(());
        }

        let mut missing_ids: String = "".to_string();

        for (id, _) in self.entries.iter() {
            missing_ids.push_str(&format!("{}, ", id));
        }

        missing_ids = missing_ids.trim_end_matches(", ").to_string();

        Err(Error::new(
            ErrorKind::IdNotTracked,
            format!("Some attribute ids were not tracked: {}", missing_ids),
        ))
    }
}
