use std::{collections::HashMap, io::Error};

use crate::attributes::attribute_id_tracker::AttributeEntry;

pub trait AttributeIdTrackerIO {
    fn read_entries(&self) -> Result<HashMap<String, AttributeEntry>, Error>;
    fn write_entry(&self, entry: &AttributeEntry) -> Result<(), Error>;
}
