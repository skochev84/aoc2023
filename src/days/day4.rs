use std::collections::HashSet;

pub fn cards(file: &str) -> Vec<String> {
    let part1 = file
        .lines()
        .map(|c| {
            let winning = process_card(c);
            if winning > 0 {
                2_i32.pow((winning - 1).try_into().unwrap())
            } else {
                0
            }
        })
        .sum::<i32>()
        .to_string();

    let mut cards = vec![1; file.lines().count()];
    for (i, c) in file.lines().enumerate() {
        let winning = process_card(c);
        for j in (i + 1)..(i + 1 + winning) {
            cards[j] += cards[i];
        }
    }
    let part2 = cards.iter().sum::<usize>().to_string();

    vec![part1, part2]
}

fn process_card(card: &str) -> usize {
    let mut card = card.split([':', '|']);

    let _ = card.next().unwrap_or_default();
    let num_win = card
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<HashSet<&str>>();
    let num_my = card
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<HashSet<&str>>();

    let winning = num_win.intersection(&num_my).count();
    winning
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = cards(file);

        assert_eq!(result[0], "13");
    }

    #[test]
    fn test_part2() {
        let file = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = cards(file);

        assert_eq!(result[1], "30");
    }
}
