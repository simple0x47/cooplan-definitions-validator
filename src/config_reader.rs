use std::io::Error;

use crate::config::Config;

pub trait ConfigReader {
    fn read(&self) -> Result<Config, Error>;
}
