use super::Day;

pub struct Day01 {
    input: Vec<String>,
}

impl Day01 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> Vec<String> {
        s.lines().map(|e| e.to_string()).collect()
    }
}

impl Day for Day01 {
    fn first_challenge(&self) -> String {
        self.input
            .iter()
            .map(|line| first::parse_number(line))
            .sum::<i64>()
            .to_string()
    }

    fn second_challenge(&self) -> String {
        self.input
            .iter()
            .map(|line| second::parse_number(line))
            .sum::<i64>()
            .to_string()
    }
}

mod first {
    pub fn parse_number(line: &str) -> i64 {
        let digits = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        number.parse().unwrap()
    }
}

mod second {
    const SPELLED_DIGITS: &[(&str, &str)] = &[
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    // O(shame)
    pub fn parse_number(line: &str) -> i64 {
        let mut digits = SPELLED_DIGITS
            .iter()
            .flat_map(|(spelled, digit)| {
                vec![
                    line.find(spelled).map(|i| (i, digit)),
                    line.rfind(spelled).map(|i| (i, digit)),
                    line.find(digit).map(|i| (i, digit)),
                    line.rfind(digit).map(|i| (i, digit)),
                ]
            })
            .flatten()
            .collect::<Vec<_>>();

        digits.sort_by_key(|(i, _)| i.to_owned());

        let number = format!("{}{}", digits.first().unwrap().1, digits.last().unwrap().1);
        number.parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let day = Day01::load(input);
        assert_eq!(day.first_challenge(), "142");
    }

    #[test]
    fn test_second_challenge() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let day = Day01::load(input);
        assert_eq!(day.second_challenge(), "281");
    }
}
