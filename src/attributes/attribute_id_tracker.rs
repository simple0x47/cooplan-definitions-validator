use std::collections::HashMap;

use crate::attributes::attribute::Attribute;
use crate::error::{Error, ErrorKind};

pub struct AttributeEntry {
    pub id: String,
    pub attribute_type: String,
}

pub struct AttributeIdTracker {
    entries: HashMap<String, AttributeEntry>,
    found_entries: HashMap<String, AttributeEntry>,
}

impl AttributeIdTracker {
    pub fn new(entries: HashMap<String, AttributeEntry>) -> AttributeIdTracker {
        AttributeIdTracker {
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
                                ).as_str(),
                            ));
                        }

                        self.found_entries.insert(id.to_string(), entry);

                        Ok(())
                    }
                    None => {
                        if self.found_entries.contains_key(id) {
                            return Err(Error::new(
                                ErrorKind::DuplicatedId,
                                format!("Duplicated attribute id {}", id).as_str(),
                            ));
                        }

                        Err(Error::new(
                            ErrorKind::IdNotFound,
                            format!("Attribute with id {} does not exist", id).as_str(),
                        ))
                    }
                }
            }
            None => Err(Error::new(
                ErrorKind::MissingId,
                "Attribute has no id".to_string().as_str(),
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
            format!("Some attribute ids were not tracked: {}", missing_ids).as_str(),
        ))
    }
}
