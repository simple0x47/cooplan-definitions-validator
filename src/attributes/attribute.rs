use serde::{Deserialize, Serialize};

use super::source_attribute::SourceAttribute;

/// Complete, in-memory attribute.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub id: String,
    pub name: String,
    pub data_type: String,
    pub unit: Option<String>,
    pub optional: bool,
}

impl Attribute {
    pub fn to_source_attribute(&self) -> SourceAttribute {
        SourceAttribute {
            id: Some(self.id.clone()),
            name: self.name.clone(),
            data_type: self.data_type.clone(),
            unit: self.unit.clone(),
            optional: Some(self.optional),
        }
    }

    pub fn to_source_attributes(attributes: &[Attribute]) -> Vec<SourceAttribute> {
        let mut source_attributes = Vec::new();

        for attribute in attributes {
            source_attributes.push(attribute.to_source_attribute());
        }

        source_attributes
    }
}
