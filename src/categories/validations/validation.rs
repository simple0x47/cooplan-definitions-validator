use std::cell::RefCell;
use std::rc::Rc;

use cooplan_definitions_lib::category::Category;

use crate::error::Error;

pub trait Validation {
    fn validate(&self, root_categories: &[Rc<RefCell<Category>>]) -> Result<(), Error>;
}
