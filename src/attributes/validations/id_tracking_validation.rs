use std::collections::HashMap;

use crate::{
    attributes::{
        attribute::Attribute, attribute_id_tracker::AttributeIdTracker,
        attribute_tracker_io::AttributeEntry, validations::validation::Validation,
    },
    error::Error,
};

pub struct IdTrackingValidation {
    attribute_id_tracker: AttributeIdTracker,
}

impl IdTrackingValidation {
    pub fn new(entries: &HashMap<String, AttributeEntry>) -> IdTrackingValidation {
        IdTrackingValidation {
            attribute_id_tracker: AttributeIdTracker::new(entries),
        }
    }
}

impl Validation for IdTrackingValidation {
    fn partially_validate(&mut self, attributes: &[Attribute]) -> Result<(), Error> {
        for attribute in attributes {
            match self.attribute_id_tracker.track_attribute(&attribute.id) {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn complete(&mut self) -> Result<(), Error> {
        match self.attribute_id_tracker.close() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }
}
