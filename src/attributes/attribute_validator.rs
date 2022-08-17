use crate::{
    attributes::attribute::Attribute,
    error::{Error, ErrorKind},
};

pub struct AttributeValidator {
    valid_data_types: Vec<String>,
}

impl AttributeValidator {
    pub fn new(valid_data_types: Vec<String>) -> AttributeValidator {
        AttributeValidator {
            valid_data_types: valid_data_types,
        }
    }

    fn is_data_type_valid(&self, attribute: &Attribute) -> bool {
        for valid_data_type in &self.valid_data_types {
            if valid_data_type.eq(&attribute.data_type) {
                return true;
            }
        }

        false
    }

    pub fn check_attribute_validity(&self, attribute: &Attribute) -> Result<(), Error> {
        let attribute_identifier: String = attribute.id.as_ref().unwrap_or(&attribute.name).clone();

        if !self.is_data_type_valid(attribute) {
            return Err(Error::new(
                ErrorKind::InvalidDataType,
                format!(
                    "Attribute '{}' has an invalid data type: {}",
                    attribute_identifier, attribute.data_type
                ),
            ));
        }

        Ok(())
    }
}
