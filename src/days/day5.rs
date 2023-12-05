use itertools::Itertools;
use std::collections::HashMap;

pub fn seeds(file: &str) -> Vec<String> {
    let mut input = file.lines();
    let seeds: Vec<u128> = input
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap_or(0))
        .skip(1)
        .collect();
    input.next();

    let Almanac(almanac, maps) = compile_almanac(input);

    let part1 = calculate_seeds(seeds, maps, almanac);

    //let ranged_seeds = seeds.iter().map_windows(|[x, y]| x..y);
    //let ranged_seeds = seeds.iter().map_windows(|[x, y]| x..y);

    for (i, c) in seeds.iter().enumerate().step_by(2) {
        c..c
    }

    let part2 = cards.iter().sum::<usize>().to_string();

    vec![part1, part2]
}

fn calculate_seeds(
    seeds: Vec<u128>,
    maps: Vec<&str>,
    almanac: HashMap<&str, Vec<(u128, u128, u128)>>,
) -> String {
    let part1 = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(seed.to_owned(), |acc, m| {
                let ranges = almanac
                    .get(m)
                    .unwrap_or(&vec![(0_u128, 0_u128, 0_u128); 0])
                    .iter()
                    .find(|r| (r.1..(r.1 + r.2)).contains(&acc))
                    .unwrap_or(&(0_u128, 0_u128, 0_u128))
                    .to_owned();

                acc - ranges.1 + ranges.0
            })
        })
        .min()
        .unwrap_or(0)
        .to_string();
    part1
}

struct Almanac<'a>(HashMap<&'a str, Vec<(u128, u128, u128)>>, Vec<&'a str>);

fn compile_almanac(input: std::str::Lines<'_>) -> Almanac {
    let mut almanac: HashMap<&str, Vec<(u128, u128, u128)>> = HashMap::new();
    let mut maps: Vec<&str> = vec![];

    input.for_each(|l| {
        if let Some((a, b, c)) = l
            .split_whitespace()
            .map(|n| n.parse::<u128>().unwrap_or(0))
            .collect_tuple()
        {
            almanac
                .entry(maps.last().unwrap_or(&""))
                .and_modify(|e| e.push((a, b, c)))
                .or_insert(vec![(a, b, c)]);
        } else if !l.is_empty() {
            maps.push(l);
        }
    });
    Almanac(almanac, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = seeds(file);

        assert_eq!(result[0], "35");
    }

    #[test]
    fn test_part2() {
        let file = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let result = seeds(file);

        assert_eq!(result[1], "46");
    }
}
