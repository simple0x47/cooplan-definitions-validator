use std::{collections::HashMap, io::Error};

use crate::categories::category_id_tracker::CategoryEntry;

pub trait CategoryIdTrackerIO {
    fn read_entries(&self) -> Result<HashMap<String, CategoryEntry>, Error>;
    fn write_entry(&self, entry: &CategoryEntry) -> Result<(), Error>;
}
