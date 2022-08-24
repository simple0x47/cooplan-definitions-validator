use std::cell::RefCell;
use std::rc::Rc;

use crate::categories::in_memory_category::InMemoryCategory;
use crate::error::Error;

pub trait Validation {
    fn validate(&self, root_categories: &[Rc<RefCell<InMemoryCategory>>]) -> Result<(), Error>;
}