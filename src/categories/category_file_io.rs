use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::DirEntry;
use std::io::{Error, ErrorKind};
use std::ops::Deref;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::category_io::CategoryIO;
use crate::categories::source_category::SourceCategory;

const CATEGORIES_DIRECTORY: &str = "./categories/";

/// Abstract wrapping of a file containing the definition of a category.
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
    fn read(&mut self) -> Result<SourceCategory, Error> {
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

    fn write(&self, category: Rc<RefCell<Category>>) -> Result<(), Error> {
        match serde_json::to_string_pretty("a") {
            Ok(json_category) => std::fs::write(self.path.clone(), json_category),
            Err(error) => {
                return Err(Error::new(ErrorKind::InvalidData, format!("{}", error)));
            }
        }
    }

    fn parent_name(&self) -> Result<Option<String>, Error> {
        let a = 5;
        let strin = self.path.clone();

        let res = strin.trim_start_matches(CATEGORIES_DIRECTORY);
        let split: Vec<&str> = res.split("/").collect();

        let mut prev_file: usize = 0;
        for s in split.as_slice() {
            if s.contains(".json") {
                break;
            }

            prev_file += 1;
        }

        if prev_file > 0 {
            return Ok(Some(split.get(prev_file - 1).unwrap().to_string()));
        }

        Ok(None)
    }
}

fn build_for_path(path: &str) -> Result<Vec<Box<dyn CategoryIO>>, Error> {
    const CATEGORY_FILE_EXTENSION: &str = ".json";

    let mut categories_files_io: Vec<Box<dyn CategoryIO>> = Vec::new();
    match std::fs::read_dir(path) {
        Ok(read) => {
            let mut directories: Vec<DirEntry> = Vec::new();

            for entry_result in read {
                let entry = entry_result.unwrap();

                match entry.file_type() {
                    Ok(file_type) => match entry.path().to_str() {
                        Some(path) => {
                            if file_type.is_dir() {
                                directories.push(entry);
                            } else if path.ends_with(CATEGORY_FILE_EXTENSION) {
                                categories_files_io.push(Box::new(CategoryFileIO::new(path)));
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

            for directory in directories {
                match directory.path().to_str() {
                    Some(path) => {
                        match build_for_path(path) {
                            Ok(mut child_categories_files_io) => {
                                let current_size = categories_files_io.len();
                                let child_size = categories_files_io.len();

                                // Detect overflows.
                                if current_size.checked_add(child_size).is_none() {
                                    return Err(Error::new(
                                        ErrorKind::Other,
                                        "cannot append vectors due to overflow",
                                    ));
                                }

                                categories_files_io.append(&mut child_categories_files_io);
                            }
                            Err(error) => return Err(error),
                        }
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "failed to retrieve path of directory".to_string(),
                        ));
                    }
                }
            }

            Ok(categories_files_io)
        }
        Err(error) => Err(error),
    }
}

/// Creates instances of `CategoryFileIO` for each file that has been found to be a category definition.
///
/// # Returns
///
/// `Ok`: vector containing instances of `CategoryFileIO` for each category definition file that has been found.
/// `Err`: error detailing why the function has failed.
pub fn build_for_all_categories() -> Result<Vec<Box<dyn CategoryIO>>, Error> {
    build_for_path(CATEGORIES_DIRECTORY)
}
