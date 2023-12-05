use std::{error::Error, fs::read_to_string};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod temp;

pub fn results() -> Result<(), Box<dyn Error>> {
    print_day("0: Temp", temp::temp(&read_to_string("./inputs/temp.txt")?));

    print_day(
        "1: Trebuchet?!",
        day1::trebuchet(&read_to_string("./inputs/trebuchet.txt")?),
    );

    print_day(
        "2: Cube Conundrum",
        day2::cubes(&read_to_string("./inputs/cubes.txt")?),
    );

    print_day(
        "3: Gear Ratios",
        day3::parts(&read_to_string("./inputs/parts.txt")?),
    );

    print_day(
        "4: Scratchcards",
        day4::cards(&read_to_string("./inputs/cards.txt")?),
    );

    print_day(
        "5: If You Give A Seed A Fertilizer",
        day5::seeds(&read_to_string("./inputs/seeds.txt")?),
    );

    Ok(())
}

fn print_day(title: &str, parts: Vec<String>) {
    println!("--- Day {} ---", title);
    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {part}", i + 1);
    }
    println!(" ")
}
