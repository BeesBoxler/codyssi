use std::fs::read_to_string;

mod days;
mod problem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    days::run();
    Ok(())
}
