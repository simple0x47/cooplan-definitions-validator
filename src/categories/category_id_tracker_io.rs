use std::{collections::HashMap, io::Error};

use crate::categories::category_id_tracker::CategoryEntry;

/// Basic utilities for interacting with the category ID tracking storage.
pub trait CategoryIdTrackerIO {
    fn read_entries(&self) -> Result<HashMap<String, CategoryEntry>, Error>;
    fn write_entry(&self, entry: &CategoryEntry) -> Result<(), Error>;
}
