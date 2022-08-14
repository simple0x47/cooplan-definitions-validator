use std::io::Error;

pub trait CategoryReader {
    /// Reads all available categories as JSON strings.
    ///
    /// # Returns
    /// The categories serialized as JSON contained within a vector.
    fn read_json_category(&mut self) -> Result<Vec<String>, Error>;
}
