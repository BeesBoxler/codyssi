use std::fs::read_to_string;

mod days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input.txt")?;

    days::run_day(&input);
    Ok(())
}
