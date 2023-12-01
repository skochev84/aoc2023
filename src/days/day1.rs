use regex::{Regex, RegexBuilder};

pub fn trebuchet(file: &str) -> Vec<String> {
    let part1 = RegexBuilder::new(r"^\D*(\d?).*?(\d?)\D*$")
        .multi_line(true)
        .build()
        .unwrap();

    let part2 = RegexBuilder::new(&format!(
        r"^\D*?({numbers}).*({numbers})\D*$|{numbers}",
        numbers = r"one|two|three|four|five|six|seven|eight|nine|\d"
    ))
    .multi_line(true)
    .build()
    .unwrap();

    vec![sum_calibration(part1, file), sum_calibration(part2, file)]
}

fn sum_calibration(part: Regex, file: &str) -> String {
    part.captures_iter(file)
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
        .sum::<i32>()
        .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = trebuchet(file);

        assert_eq!(result[0], "142");
    }

    #[test]
    fn test_part2() {
        let file = r"two1nine
0
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = trebuchet(file);

        assert_eq!(result[1], "281");
    }
}
