use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

pub fn cubes(file: &str) -> Vec<String> {
    let part1 = RegexBuilder::new(r"(?:(Game) (\d+):)|.*(?: (\d+) (red|green|blue)).*")
        .swap_greed(true)
        .build()
        .unwrap();

    let mut max_colors = HashMap::new();
    max_colors.insert("red", 12);
    max_colors.insert("green", 13);
    max_colors.insert("blue", 14);

    vec![
        sum_game_ids(part1.clone(), file, max_colors),
        sum_max_power(part1, file),
    ]
}

fn sum_game_ids(part: Regex, file: &str, max_colors: HashMap<&str, usize>) -> String {
    let mut game_ids = Vec::<bool>::new();
    game_ids.push(false);
    let mut game_id = 0;

    part.captures_iter(file).for_each(|c| {
        let current_game_id = c.get(2).map_or("", |m| m.as_str());
        if current_game_id.is_empty() {
            let cube_count = c
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap_or_default();
            let cube_colour = c.get(4).map_or("", |m| m.as_str());
            if cube_count > *max_colors.get(cube_colour).unwrap() {
                game_ids[game_id] = false;
            }
        } else {
            game_id = current_game_id.parse::<usize>().unwrap_or(0);
            game_ids.push(true);
        }
    });
    let mut sum = 0;

    for (i, gid) in game_ids.into_iter().enumerate() {
        if gid {
            sum += i;
        }
    }
    sum.to_string()
}

fn sum_max_power(part: Regex, file: &str) -> String {
    let mut max_colors = HashMap::new();
    max_colors.insert("red", 0);
    max_colors.insert("green", 0);
    max_colors.insert("blue", 0);

    let mut game_id = 0;
    let mut power_sum = 0;

    part.captures_iter(file).for_each(|c| {
        let current_game_id = c.get(2).map_or("", |m| m.as_str());
        if current_game_id.is_empty() {
            let cube_count = c
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap_or_default();
            let cube_colour = c.get(4).map_or("", |m| m.as_str());
            if cube_count > *max_colors.get(cube_colour).unwrap() {
                max_colors.insert(cube_colour, cube_count);
            }
        } else {
            power_sum += max_colors.get("red").unwrap_or(&1)
                * max_colors.get("green").unwrap_or(&1)
                * max_colors.get("blue").unwrap_or(&1);
            game_id = current_game_id.parse::<usize>().unwrap_or(0);

            max_colors.insert("red", 1);
            max_colors.insert("green", 1);
            max_colors.insert("blue", 1);
        }
    });

    power_sum += max_colors.get("red").unwrap_or(&1)
        * max_colors.get("green").unwrap_or(&1)
        * max_colors.get("blue").unwrap_or(&1);

    power_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = cubes(file);

        assert_eq!(result[0], "8");
    }

    #[test]
    fn test_part2() {
        let file = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = cubes(file);

        assert_eq!(result[1], "2286");
    }
}
