use std::{collections::HashMap, io::Error};

#[derive(Clone)]
pub struct AttributeEntry {
    pub id: String,
    pub data_type: String,
}

pub trait AttributeTrackerIO {
    fn read_entries(&self) -> Result<HashMap<String, AttributeEntry>, Error>;
    fn write_entry(&self, entry: &AttributeEntry) -> Result<(), Error>;
}
