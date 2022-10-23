use pairs_rs::start_ui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    start_ui()?;
    Ok(())
}
