use std::time::{Duration, Instant};
use std::{fs::read_to_string, path::Path};

type DayLoader = fn(&str) -> Box<dyn Day>;

mod day_00;
mod day_01;

const DAY_LOADERS: &[DayLoader] = &[
    |input| Box::new(day_00::Day00::load(input)),
    |input| Box::new(day_01::Day01::load(input)),
];

pub trait Day {
    fn first_challenge(&self) -> String;
    fn second_challenge(&self) -> String;
}

pub struct DayResult {
    pub number: usize,
    pub load_elapsed: Duration,
    pub first_result: String,
    pub first_elapsed: Duration,
    pub second_result: String,
    pub second_elapsed: Duration,
}

pub type Report = Vec<DayResult>;
pub type ReportSlice<'a> = &'a [DayResult];

pub struct Advent {
    input_folder: String,
}

macro_rules! elapsed {
    ($expression:expr) => {{
        let clock = Instant::now();
        let result = $expression;
        (result, clock.elapsed())
    }};
}

impl Advent {
    pub fn new(input_folder: String) -> Self {
        Self { input_folder }
    }

    pub fn run_all_days(&self) -> Report {
        (1..DAY_LOADERS.len())
            .map(|number| self.run_day(number))
            .collect()
    }

    pub fn run_day(&self, number: usize) -> DayResult {
        let loader = DAY_LOADERS
            .get(number)
            .unwrap_or_else(|| panic!("Day {number} not found!"));

        let input = self.read_input(number);

        let (day, load_elapsed) = elapsed!(loader(&input));
        let (first_result, first_elapsed) = elapsed!(day.first_challenge());
        let (second_result, second_elapsed) = elapsed!(day.second_challenge());

        DayResult {
            number,
            first_result,
            second_result,
            load_elapsed,
            first_elapsed,
            second_elapsed,
        }
    }

    fn read_input(&self, number: usize) -> String {
        let input_path = &Path::new(&self.input_folder)
            .join(format!("day_{:02}.txt", number))
            .to_str()
            .unwrap()
            .to_owned();

        read_to_string(input_path).expect("Load input failed")
    }
}
