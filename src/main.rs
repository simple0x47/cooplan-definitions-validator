mod attributes;
mod categories;
mod error;
mod tests;
mod validator;

use std::collections::HashMap;

use categories::{
    category_file_reader::CategoryFileReader,
    category_json_deserialize::deserialize_categories_from_json_strings,
    category_reader::CategoryReader,
};

fn main() {
    let category_entries: HashMap<String, crate::categories::category_id_tracker::CategoryEntry> =
        HashMap::new();
    let mut category_tracker =
        crate::categories::category_id_tracker::CategoryIdTracker::new(category_entries);

    let attribute_entries: HashMap<
        String,
        crate::attributes::attribute_id_tracker::AttributeEntry,
    > = HashMap::new();
    let mut attribute_tracker =
        crate::attributes::attribute_id_tracker::AttributeIdTracker::new(attribute_entries);

    let mut reader: CategoryFileReader = CategoryFileReader::new();

    match reader.read_json_category() {
        Ok(categories_json) => match deserialize_categories_from_json_strings(&categories_json) {
            Ok(categories) => {
                for mut category in categories {
                    if category.id.is_none() {
                        match crate::categories::category_id_generator::set_random_id(category) {
                            Ok(cat) => category = cat,
                            Err(error) => {
                                println!("{}", error);
                                std::process::exit(1);
                            }
                        }
                    }

                    match category_tracker.track_category(&category) {
                        Ok(_) => {
                            for mut attribute in category.attributes {
                                if attribute.id.is_none() {
                                    match crate::attributes::attribute_id_generator::set_random_id(
                                        attribute,
                                    ) {
                                        Ok(attr) => attribute = attr,
                                        Err(error) => {
                                            println!("{}", error);
                                            std::process::exit(1);
                                        }
                                    }
                                }

                                match attribute_tracker.track_attribute(&attribute) {
                                    Ok(_) => (),
                                    Err(error) => {
                                        println!("{}", error);
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
            }
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
