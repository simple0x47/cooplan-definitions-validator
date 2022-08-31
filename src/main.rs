use validator::Validator;

mod attributes;
mod categories;
mod error;
mod tests;

mod config;
mod config_file_reader;
mod config_reader;
pub mod validator;

fn main() {
    let mut validator = Validator::new();

    match validator.validate() {
        Ok(_) => (),
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
