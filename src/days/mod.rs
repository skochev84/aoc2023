use std::{error::Error, fs::read_to_string};

mod day1;
mod day2;
mod day3;
mod temp;

pub fn results() -> Result<(), Box<dyn Error>> {
    print_day("0: Temp", temp::temp(&read_to_string("temp.txt")?));

    print_day(
        "1: Trebuchet?!",
        day1::trebuchet(&read_to_string("trebuchet.txt")?),
    );

    print_day(
        "2: Cube Conundrum",
        day2::cubes(&read_to_string("cubes.txt")?),
    );

    print_day("3: Gear Ratios", day3::parts(&read_to_string("parts.txt")?));

    Ok(())
}

fn print_day(title: &str, parts: Vec<String>) {
    println!("--- Day {} ---", title);
    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {part}", i + 1);
    }
    println!(" ")
}
