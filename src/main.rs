mod attributes;
mod categories;
mod error;
mod tests;

mod config;
mod config_file_reader;
mod config_reader;

use attributes::{
    attribute_id_tracker::{AttributeEntry, AttributeIdTracker},
    attribute_id_tracker_file_io::AttributeIdTrackerFileIO,
    attribute_id_tracker_io::AttributeIdTrackerIO,
};
use categories::{
    category_file_io::build_for_all_categories,
    category_id_tracker::{CategoryEntry, CategoryIdTracker},
    category_id_tracker_file_io::CategoryIdTrackerFileIO,
    category_id_tracker_io::CategoryIdTrackerIO,
    category_io::CategoryIO,
};
use config_file_reader::ConfigFileReader;

use crate::{attributes::attribute_validator::AttributeValidator, config_reader::ConfigReader};

fn initialize_attribute_validator() -> AttributeValidator {
    const CONFIG_FILE_PATH: &str = "./config.json";
    let config_reader = ConfigFileReader::new(CONFIG_FILE_PATH);

    match config_reader.read() {
        Ok(config) => {
            let attribute_validator = AttributeValidator::new(config.valid_data_types());

            return attribute_validator;
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}

fn write_new_ids() {
    let category_id_tracker_io = CategoryIdTrackerFileIO::new();
    let attribute_id_tracker_io = AttributeIdTrackerFileIO::new();

    match build_for_all_categories() {
        Ok(categories_files_io) => {
            for mut category_file_io in categories_files_io {
                match category_file_io.read() {
                    Ok(mut category) => {
                        let mut was_category_updated: bool = false;

                        if category.id.is_none() {
                            match crate::categories::category_id_generator::set_random_id(category)
                            {
                                Ok(cat) => {
                                    category = cat;
                                    match &category.id {
                                        Some(id) => {
                                            was_category_updated = true;

                                            match category_id_tracker_io
                                                .write_entry(&CategoryEntry { id: id.clone() })
                                            {
                                                Ok(_) => (),
                                                Err(error) => {
                                                    println!(
                                                        "[{}] {}",
                                                        category_file_io.path(),
                                                        error
                                                    );
                                                    std::process::exit(1);
                                                }
                                            }
                                        }
                                        None => {
                                            println!(
                                                "[{}]Unexpected: Category id is None",
                                                category_file_io.path()
                                            );
                                            std::process::exit(1);
                                        }
                                    }
                                }
                                Err(error) => {
                                    println!("[{}] {}", category_file_io.path(), error);
                                    std::process::exit(1);
                                }
                            }
                        }

                        let attributes = category.attributes;
                        category.attributes = Vec::new();

                        for mut attribute in attributes {
                            if attribute.id.is_none() {
                                match crate::attributes::attribute_id_generator::set_random_id(
                                    attribute,
                                ) {
                                    Ok(attr) => {
                                        attribute = attr;

                                        match &attribute.id {
                                            Some(id) => {
                                                was_category_updated = true;

                                                match attribute_id_tracker_io.write_entry(
                                                    &AttributeEntry {
                                                        id: id.clone(),
                                                        attribute_type: attribute.data_type.clone(),
                                                    },
                                                ) {
                                                    Ok(_) => {}
                                                    Err(error) => {
                                                        println!(
                                                            "[{}] {}",
                                                            category_file_io.path(),
                                                            error
                                                        );
                                                        std::process::exit(1);
                                                    }
                                                }
                                            }
                                            None => {
                                                println!(
                                                    "[{}] Unexpected: Attribute id is None",
                                                    category_file_io.path()
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        println!("[{}] {}", category_file_io.path(), error);
                                        std::process::exit(1);
                                    }
                                }
                            }

                            category.attributes.push(attribute);
                        }

                        if was_category_updated {
                            match category_file_io.write(&category) {
                                Ok(_) => (),
                                Err(error) => {
                                    println!("[{}] {}", category_file_io.path(), error);
                                    std::process::exit(1);
                                }
                            }
                        }
                    }
                    Err(error) => {
                        println!("[{}] {}", category_file_io.path(), error);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}

fn validate_categories(attributes_validator: AttributeValidator) {
    let category_id_tracker_io = CategoryIdTrackerFileIO::new();
    let attribute_id_tracker_io = AttributeIdTrackerFileIO::new();

    match category_id_tracker_io.read_entries() {
        Ok(category_entries) => {
            let mut category_id_tracker = CategoryIdTracker::new(category_entries);

            match attribute_id_tracker_io.read_entries() {
                Ok(attribute_entries) => {
                    let mut attribute_id_tracker = AttributeIdTracker::new(attribute_entries);

                    match build_for_all_categories() {
                        Ok(categories_files_io) => {
                            for mut category_file_io in categories_files_io {
                                match category_file_io.read() {
                                    Ok(mut category) => {
                                        if category.id.is_none() {
                                            continue;
                                        }

                                        match category_id_tracker.track_category(&category) {
                                            Ok(_) => {
                                                let attributes = category.attributes;
                                                category.attributes = Vec::new();

                                                for attribute in attributes {
                                                    match attributes_validator
                                                        .check_attribute_validity(&attribute)
                                                    {
                                                        Ok(_) => {}
                                                        Err(error) => {
                                                            println!("{}", error);
                                                            std::process::exit(1);
                                                        }
                                                    }

                                                    if attribute.id.is_none() {
                                                        continue;
                                                    }

                                                    match attribute_id_tracker
                                                        .track_attribute(&attribute)
                                                    {
                                                        Ok(_) => {
                                                            category.attributes.push(attribute);
                                                        }
                                                        Err(error) => {
                                                            println!("{}", error);
                                                            std::process::exit(1);
                                                        }
                                                    }
                                                }
                                            }
                                            Err(error) => {
                                                println!("[{}] {}", category_file_io.path(), error);
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        println!("[{}] {}", category_file_io.path(), error);
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            println!("{}", error);
                            std::process::exit(1);
                        }
                    }

                    match category_id_tracker.close() {
                        Ok(_) => match attribute_id_tracker.close() {
                            Ok(_) => {}
                            Err(error) => {
                                println!("{}", error);
                                std::process::exit(1);
                            }
                        },
                        Err(error) => {
                            println!("{}", error);
                            std::process::exit(1);
                        }
                    }
                }
                Err(error) => {
                    println!("{}", error);
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}

fn main() {
    let attributes_validator = initialize_attribute_validator();
    validate_categories(attributes_validator);
    write_new_ids();
}
