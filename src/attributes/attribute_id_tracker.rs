use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::{Error, ErrorKind};

use super::attribute_tracker_io::AttributeEntry;

static mut instance: Option<Rc<RefCell<AttributeIdTracker>>> = None;

pub struct AttributeIdTracker {
    entries: HashMap<String, AttributeEntry>,
    found_entries: Vec<String>,
}

impl AttributeIdTracker {
    pub fn new(entries: &HashMap<String, AttributeEntry>) -> AttributeIdTracker {
        AttributeIdTracker {
            entries: HashMap::clone(entries),
            found_entries: Vec::new(),
        }
    }

    /// Tracks the id of the attribute.
    ///
    /// Error kinds:
    ///
    /// * `MissingId` - if the attribute has no id.
    /// * `IdNotFound`- if the attribute's id could not be found.
    /// * `DuplicatedId` - if the attribute's id is duplicated.
    ///
    /// # Arguments
    ///
    /// * `attribute` - Attribute whose id is going to be tracked.
    pub fn track_attribute(&mut self, id: &str) -> Result<(), Error> {
        let entry = self.entries.remove(id);

        match entry {
            Some(entry) => {
                self.found_entries.push(id.to_string());

                Ok(())
            }
            None => {
                if self.found_entries.contains(&id.to_string()) {
                    return Err(Error::new(
                        ErrorKind::DuplicatedId,
                        format!("duplicated attribute id {}", id).as_str(),
                    ));
                }

                Err(Error::new(
                    ErrorKind::IdNotFound,
                    format!("attribute with id {} does not exist", id).as_str(),
                ))
            }
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
            format!("some attribute ids were not tracked: {}", missing_ids).as_str(),
        ))
    }
}
