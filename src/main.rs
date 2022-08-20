mod attributes;
mod categories;
mod error;
mod tests;

mod config;
mod config_file_reader;
mod config_reader;

use categories::category_validator::validate_categories;

fn main() {
    match validate_categories() {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    }
}
