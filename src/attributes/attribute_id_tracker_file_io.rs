use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use crate::attributes::attribute_id_tracker_io::AttributeIdTrackerIO;

use super::attribute_id_tracker::AttributeEntry;

pub struct AttributeIdTrackerFileIO {}

const ID_TRACKER_PATH: &str = "./attribute_id_tracker.csv";

impl AttributeIdTrackerFileIO {
    pub fn new() -> AttributeIdTrackerFileIO {
        AttributeIdTrackerFileIO {}
    }
}

impl AttributeIdTrackerIO for AttributeIdTrackerFileIO {
    fn read_entries(&self) -> Result<HashMap<String, AttributeEntry>, std::io::Error> {
        const SPLIT_PATTERN: &str = ";";
        let mut entries: HashMap<String, AttributeEntry> = HashMap::new();

        match std::fs::read_to_string(ID_TRACKER_PATH) {
            Ok(content) => {
                for line in content.lines() {
                    if line.is_empty() {
                        continue;
                    }

                    let result: Vec<&str> = line.split(SPLIT_PATTERN).collect();

                    if result.len() <= 0 || result.len() > 2 {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Attribute id tracker file contains a malformed line.",
                        ));
                    }

                    entries.insert(
                        result[0].to_string(),
                        AttributeEntry {
                            id: result[0].to_string(),
                            attribute_type: result[1].to_string(),
                        },
                    );
                }
            }
            Err(error) => {
                return Err(error);
            }
        }

        Ok(entries)
    }

    fn write_entry(&self, entry: &AttributeEntry) -> Result<(), std::io::Error> {
        match std::fs::read_to_string(ID_TRACKER_PATH) {
            Ok(current_content) => {
                let new_content =
                    format!("{}\n{};{}", current_content, entry.id, entry.attribute_type);

                match std::fs::write(ID_TRACKER_PATH, new_content) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
}
