use std::{collections::HashMap, io::Error};

use crate::categories::category_id_tracker_io::CategoryIdTrackerIO;

use super::category_id_tracker::CategoryEntry;

const ID_TRACKER_PATH: &str = "./category_id_tracker.csv";

pub struct CategoryIdTrackerFileIO {}

impl CategoryIdTrackerFileIO {
    pub fn new() -> CategoryIdTrackerFileIO {
        CategoryIdTrackerFileIO {}
    }
}

impl CategoryIdTrackerIO for CategoryIdTrackerFileIO {
    fn read_entries(&self) -> Result<HashMap<String, CategoryEntry>, Error> {
        let mut entries: HashMap<String, CategoryEntry> = HashMap::new();

        match std::fs::read_to_string(ID_TRACKER_PATH) {
            Ok(content) => {
                for id in content.lines() {
                    if id.is_empty() {
                        continue;
                    }

                    entries.insert(id.to_string(), CategoryEntry { id: id.to_string() });
                }
            }
            Err(error) => {
                return Err(error);
            }
        }

        Ok(entries)
    }

    fn write_entry(&self, entry: &CategoryEntry) -> Result<(), Error> {
        match std::fs::read_to_string(ID_TRACKER_PATH) {
            Ok(current_content) => {
                let new_content = format!("{}\n{}", current_content, entry.id);

                match std::fs::write(ID_TRACKER_PATH, new_content) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
}
