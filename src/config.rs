use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    valid_data_types: Vec<String>,
    reserved_keywords: Vec<String>,
}

impl Config {
    pub fn valid_data_types(&self) -> Vec<String> {
        let mut valid_data_types_copy: Vec<String> = Vec::new();

        for valid_data_type in &self.valid_data_types {
            valid_data_types_copy.push(valid_data_type.clone());
        }

        valid_data_types_copy
    }

    pub fn reserved_keywords(&self) -> Vec<String> {
        let mut reserved_keywords_copy: Vec<String> = Vec::new();

        for reserved_keyword in &self.reserved_keywords {
            reserved_keywords_copy.push(reserved_keyword.clone());
        }

        reserved_keywords_copy
    }
}
