use std::{error::Error, fs::read_to_string};

mod day1;
mod temp;

pub fn results() -> Result<(), Box<dyn Error>> {
    print_day("0: Temp", temp::temp(&read_to_string("temp.txt")?));

    print_day(
        "1: Trebuchet?!",
        day1::trebuchet(&read_to_string("trebuchet.txt")?),
    );

    Ok(())
}

fn print_day(title: &str, parts: Vec<String>) {
    println!("--- Day {} ---", title);
    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {part}", i + 1);
    }
    println!()
}
