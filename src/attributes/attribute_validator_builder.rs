use std::io::Error;

use crate::{
    attributes::attribute_validator::AttributeValidator, config_file_reader::ConfigFileReader,
    config_reader::ConfigReader,
};

pub fn build_attribute_validator() -> Result<AttributeValidator, Error> {
    const CONFIG_FILE_PATH: &str = "./config.json";
    let config_reader = ConfigFileReader::new(CONFIG_FILE_PATH);

    match config_reader.read() {
        Ok(config) => {
            let attribute_validator = AttributeValidator::new(config.valid_data_types());

            return Ok(attribute_validator);
        }
        Err(error) => Err(error),
    }
}
