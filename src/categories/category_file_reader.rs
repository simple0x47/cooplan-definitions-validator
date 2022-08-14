use crate::categories::category_reader::CategoryReader;
use std::{
    fs,
    io::{Error, ErrorKind},
};

pub struct CategoryFileReader {
    files_to_be_read: Vec<String>,
}

impl CategoryFileReader {
    pub fn new() -> CategoryFileReader {
        CategoryFileReader {
            files_to_be_read: Vec::new(),
        }
    }

    fn read_path(&mut self, path: &str) -> Result<(), Error> {
        const CATEGORY_FILE_EXTENSION: &str = ".json";

        match fs::read_dir(path) {
            Ok(read) => {
                for entry_result in read {
                    let entry = entry_result.unwrap();

                    match entry.file_type() {
                        Ok(file_type) => match entry.path().to_str() {
                            Some(path) => {
                                if file_type.is_dir() {
                                    match self.read_path(&path) {
                                        Ok(_) => (),
                                        Err(error) => return Err(error),
                                    }
                                } else {
                                    if path.ends_with(CATEGORY_FILE_EXTENSION) {
                                        self.files_to_be_read.push(path.to_string());
                                    }
                                }
                            }
                            None => {
                                return Err(Error::new(
                                    ErrorKind::Unsupported,
                                    "Could not convert path to string.",
                                ));
                            }
                        },
                        Err(error) => return Err(error),
                    }
                }

                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn read_json_file(&self, file_path: &str) -> Result<String, Error> {
        return fs::read_to_string(file_path);
    }
}

impl CategoryReader for CategoryFileReader {
    fn read_json_category(&mut self) -> Result<Vec<String>, Error> {
        let path = "./categories/";

        match self.read_path(path) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        let mut categories: Vec<String> = Vec::new();

        for file_path in &self.files_to_be_read {
            match self.read_json_file(file_path) {
                Ok(category) => categories.push(category),
                Err(error) => return Err(error),
            }
        }

        Ok(categories)
    }
}
