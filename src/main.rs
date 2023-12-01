use std::error::Error;

mod days;

fn main() -> Result<(), Box<dyn Error>> {
    days::results()
}
