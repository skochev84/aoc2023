use regex::RegexBuilder;
use std::fs;

fn main() {
    println!("Merry Christmas!");

    let (part1, part2) = trebuchet();
    println!(
        "--- Day 1: Trebuchet?! --- \nAnswer: \nPart 1: {}\nPart 2: {}\n",
        part1, part2
    );
}

fn trebuchet() -> (i32, i32) {
    let file = read_file("trebuchet.txt");

    let part1 = RegexBuilder::new(r"^\D*(\d?).*?(\d?)\D*$")
        .multi_line(true)
        .build()
        .unwrap();

    let part2 = RegexBuilder::new(r"^\D*?(one|two|three|four|five|six|seven|eight|nine|[0-9]).*(one|two|three|four|five|six|seven|eight|nine|[0-9])\D*$|one|two|three|four|five|six|seven|eight|nine|[0-9]")
        .multi_line(true)
        .build()
        .unwrap();

    (
        part1
            .captures_iter(&file)
            .map(|c| {
                let num1 = c.get(1).map_or("", |m| m.as_str());
                let num2 = c.get(2).map_or("", |m| {
                    let num = m.as_str();
                    if num.is_empty() {
                        num1
                    } else {
                        num
                    }
                });

                (num1.to_owned() + num2).parse::<i32>().unwrap_or(0)
            })
            .sum(),
        part2
            .captures_iter(&file)
            .map(|c| {
                let num1 = word_to_number(c.get(1).map_or("", |m| m.as_str()));
                let num2 = c.get(2).map_or("", |m| {
                    let num = m.as_str();
                    if num.is_empty() {
                        num1
                    } else {
                        word_to_number(num)
                    }
                });
                if num1.is_empty() && num2.is_empty() {
                    let num = word_to_number(c.get(0).map_or("", |m| m.as_str()));
                    (num.to_owned() + num).parse::<i32>().unwrap_or(0)
                } else {
                    (num1.to_owned() + num2).parse::<i32>().unwrap_or(0)
                }
            })
            .sum(),
    )
}

fn word_to_number(word: &str) -> &str {
    match word {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => word,
    }
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
