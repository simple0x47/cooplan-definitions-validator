use std::cell::RefCell;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::error::Error;

pub trait Validation {
    fn validate(&self, root_categories: &[Rc<RefCell<Category>>]) -> Result<(), Error>;
}
