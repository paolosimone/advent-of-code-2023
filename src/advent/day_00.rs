use super::Day;

pub struct Day00 {
    input: String,
}

impl Day00 {
    pub fn load(input: &str) -> Self {
        Self {
            input: Self::parse_input(input),
        }
    }

    fn parse_input(s: &str) -> String {
        s.to_string()
    }
}

impl Day for Day00 {
    fn first_challenge(&self) -> String {
        self.input.to_string()
    }

    fn second_challenge(&self) -> String {
        "TODO".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_challenge() {
        let input = "TODO";
        let day = Day00::load(input);
        assert_eq!(day.first_challenge(), "TODO");
    }

    #[test]
    fn test_second_challenge() {
        let input = "TODO";
        let day = Day00::load(input);
        assert_eq!(day.second_challenge(), "TODO");
    }
}
