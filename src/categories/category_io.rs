use std::io::Error;

use crate::categories::category::Category;

pub trait CategoryIO {
    fn read(&mut self) -> Result<Category, Error>;
    fn write(&self, category: &Category) -> Result<(), Error>;
}
