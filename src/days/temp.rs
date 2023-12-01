pub fn temp(file: &str) -> Vec<String> {
    vec![file.to_owned()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = r"Merry Christmas!";

        let result = temp(file);

        assert_eq!(result[0], "Merry Christmas!");
    }
}
