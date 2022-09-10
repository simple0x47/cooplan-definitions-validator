use crate::{
    attributes::attribute::Attribute,
    error::{Error, ErrorKind},
};

use super::validation::Validation;

pub struct DataTypeValidation {
    failed_validations: Vec<Error>,
    valid_data_types: Vec<String>,
}

impl DataTypeValidation {
    pub fn new(valid_data_types: Vec<String>) -> DataTypeValidation {
        DataTypeValidation {
            failed_validations: Vec::new(),
            valid_data_types,
        }
    }

    fn validate_attribute_data_type(&self, attribute: &Attribute) -> Result<(), Error> {
        for valid_data_type in self.valid_data_types.as_slice() {
            if valid_data_type.eq(&attribute.data_type) {
                return Ok(());
            }
        }

        Err(Error::new(
            ErrorKind::FailedDataTypeAttributeValidation,
            format!(
                "attribute '{}' with id '{}' has an invalid data type: {}",
                attribute.name, attribute.id, attribute.data_type
            )
            .as_str(),
        ))
    }
}

impl Validation for DataTypeValidation {
    fn partially_validate(&mut self, attributes: &[Attribute]) -> Result<(), Error> {
        for attribute in attributes {
            match self.validate_attribute_data_type(attribute) {
                Ok(_) => (),
                Err(error) => self.failed_validations.push(error),
            }
        }

        Ok(())
    }

    fn complete(&mut self) -> Result<(), Error> {
        if self.failed_validations.is_empty() {
            return Ok(());
        }

        let mut error_accumulation: String = String::new();

        for error in self.failed_validations.as_slice() {
            error_accumulation.push('\n');
            error_accumulation.push_str(error.message.as_str());
        }

        Err(Error::new(
            ErrorKind::FailedDataTypeAttributeValidation,
            error_accumulation.as_str(),
        ))
    }
}
