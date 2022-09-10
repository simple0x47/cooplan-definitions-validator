use ci::CI;

mod attributes;
mod categories;
mod error;
mod tests;

pub mod ci;
mod config;
mod config_file_reader;
mod config_reader;

fn main() {
    match CI::try_new() {
        Ok(mut validator) => match validator.run_ci_logic() {
            Ok(_) => (),
            Err(error) => {
                println!("Error: {}", error);
                std::process::exit(1);
            }
        },
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
