use std::{collections::HashMap, io::Error, io::ErrorKind};

use crate::attributes::attribute::Attribute;

use uuid::Uuid;

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
    /// * `InvalidInput` - if the attribute has no id.
    /// * `NotFound`- if the attribute id could not be found.
    /// * `InvalidData` - if the attribute id is duplicated.
    /// * `Unsupported` - if the attribute type has been changed.
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
                                ErrorKind::Unsupported,
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
                                ErrorKind::InvalidData,
                                format!("Duplicated attribute id {}", id),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::NotFound,
                            format!("Attribute with id {} does not exist", id),
                        ))
                    }
                }
            }
            None => Err(Error::new(ErrorKind::InvalidInput, "Attribute has no id")),
        }
    }

    /// To be called after all attributes have been tracked.
    ///
    /// Error kinds:
    ///
    /// * `InvalidData` - if there are attribute ids that have not been tracked.
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
            ErrorKind::InvalidData,
            format!("Some attribute ids were not tracked: {}", missing_ids),
        ))
    }
}
