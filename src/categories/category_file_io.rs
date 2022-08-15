use std::io::{Error, ErrorKind};

use crate::categories::category::Category;
use crate::categories::category_io::CategoryIO;

pub struct CategoryFileIO {
    path: String,
}

impl CategoryFileIO {
    pub fn new(path: &str) -> CategoryFileIO {
        CategoryFileIO {
            path: path.to_string(),
        }
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl CategoryIO for CategoryFileIO {
    fn read(&mut self) -> Result<Category, Error> {
        match std::fs::read_to_string(self.path.clone()) {
            Ok(json_category) => match serde_json::de::from_str(json_category.as_str()) {
                Ok(category) => {
                    return Ok(category);
                }
                Err(error) => return Err(Error::new(ErrorKind::InvalidData, format!("{}", error))),
            },
            Err(error) => {
                return Err(error);
            }
        }
    }

    fn write(&self, category: &Category) -> Result<(), Error> {
        match serde_json::to_string_pretty(category) {
            Ok(json_category) => std::fs::write(self.path.clone(), json_category),
            Err(error) => {
                return Err(Error::new(ErrorKind::InvalidData, format!("{}", error)));
            }
        }
    }
}

fn build_for_path(path: &str) -> Result<Vec<CategoryFileIO>, Error> {
    const CATEGORY_FILE_EXTENSION: &str = ".json";

    let mut categories_files_io: Vec<CategoryFileIO> = Vec::new();
    match std::fs::read_dir(path) {
        Ok(read) => {
            for entry_result in read {
                let entry = entry_result.unwrap();

                match entry.file_type() {
                    Ok(file_type) => match entry.path().to_str() {
                        Some(path) => {
                            if file_type.is_dir() {
                                match build_for_path(&path) {
                                    Ok(mut child_categories_files_io) => {
                                        let current_size = categories_files_io.len();
                                        let child_size = categories_files_io.len();

                                        // Detect overflows.
                                        if current_size.checked_add(child_size).is_none() {
                                            return Err(Error::new(
                                                ErrorKind::Other,
                                                "Cannot append vectors due to overflow.",
                                            ));
                                        }

                                        categories_files_io.append(&mut child_categories_files_io);
                                    }
                                    Err(error) => return Err(error),
                                }
                            } else {
                                if path.ends_with(CATEGORY_FILE_EXTENSION) {
                                    categories_files_io.push(CategoryFileIO::new(path));
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

            Ok(categories_files_io)
        }
        Err(error) => Err(error),
    }
}

pub fn build_for_all_categories() -> Result<Vec<CategoryFileIO>, Error> {
    const CATEGORIES_DIRECTORY: &str = "./categories/";

    build_for_path(CATEGORIES_DIRECTORY)
}