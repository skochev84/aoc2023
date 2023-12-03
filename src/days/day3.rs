use regex::RegexBuilder;
use std::collections::HashMap;

pub fn parts(file: &str) -> Vec<String> {
    let part = RegexBuilder::new(r"\d+").build().unwrap();
    let line_length = file.len() / file.lines().count();

    let file_trim = &file.replace(char::is_whitespace, "");

    let mut gears: HashMap<usize, i32> = HashMap::new();

    let part1 = part
        .captures_iter(file_trim)
        .map(|c| {
            let mut result = 0;
            let num = c.get(0).unwrap();
            let mut start = num.start();
            let mut end = num.end();
            let number = num.as_str().parse::<i32>().unwrap_or(0);

            if start > 0 {
                start = start - 1
            }
            if end < file_trim.len() {
                end = end + 1
            }

            if start > line_length {
                let new_start_length = start - line_length + 2;
                let new_end_length = end - line_length + 2;

                if let Some(r) = is_part_or_gear(
                    file_trim,
                    &mut gears,
                    number,
                    new_start_length,
                    new_end_length,
                ) {
                    result = r;
                }
            }

            if let Some(r) = is_part_or_gear(file_trim, &mut gears, number, start, end) {
                result = r;
            }

            if end + line_length < file_trim.len() {
                let new_start_length = start + line_length - 2;
                let new_end_length = end + line_length - 2;

                if let Some(r) = is_part_or_gear(
                    file_trim,
                    &mut gears,
                    number,
                    new_start_length,
                    new_end_length,
                ) {
                    result = r;
                }
            }
            result
        })
        .sum::<i32>()
        .to_string();

    let part2 = gears.get(&0).unwrap_or(&0).to_string();

    vec![part1, part2]
}

fn is_part_or_gear(
    file_trim: &String,
    gears: &mut HashMap<usize, i32>,
    num: i32,
    new_start_length: usize,
    new_end_length: usize,
) -> Option<i32> {
    if file_trim[new_start_length..new_end_length]
        .contains(|c: char| !c.is_ascii_digit() && c != '.' && !c.is_ascii_whitespace())
    {
        if let Some(v) = file_trim[new_start_length..new_end_length].find('*') {
            let gear = new_start_length + v;
            match gears.get(&gear) {
                Some(g) => gears.insert(0, gears.get(&0).unwrap_or(&0) + (g * num)),
                None => gears.insert(gear, num),
            };
        }
        return Some(num);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let file = &read_to_string("./inputs/parts_test.txt").unwrap();

        let result = parts(file);

        assert_eq!(result[0], "4361");
    }

    #[test]
    fn test_part2() {
        let file = &read_to_string("./inputs/parts_test.txt").unwrap();

        let result = parts(file);

        assert_eq!(result[1], "467835");
    }
}
