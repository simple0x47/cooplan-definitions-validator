use serde::{Deserialize, Serialize};

use crate::error::Error;

use super::attribute::Attribute;

/// Partial or full, input based attribute.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SourceAttribute {
    pub id: Option<String>,
    pub name: String,
    pub data_type: String,
    pub unit: Option<String>,
    pub optional: Option<bool>,
}

impl SourceAttribute {
    pub fn to_attribute(&self) -> Result<Attribute, Error> {
        match &self.id {
            Some(id) => {
                let optional = match self.optional {
                    Some(value) => value,
                    None => false,
                };

                Ok(Attribute {
                    id: id.clone(),
                    name: self.name.clone(),
                    data_type: self.data_type.clone(),
                    unit: self.unit.clone(),
                    optional,
                })
            }
            None => Err(Error::new(
                crate::error::ErrorKind::MissingId,
                "source attribute cannot be converted to attribute without an id",
            )),
        }
    }

    pub fn to_attributes(source_attributes: &[SourceAttribute]) -> Result<Vec<Attribute>, Error> {
        let mut attributes = Vec::new();

        for source_attribute in source_attributes {
            match source_attribute.to_attribute() {
                Ok(attribute) => attributes.push(attribute),
                Err(error) => return Err(error),
            }
        }

        Ok(attributes)
    }
}
