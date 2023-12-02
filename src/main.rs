use std::path::Path;

use advent::Advent;
use output::{display_table, MERRY_CHRISTMAS};

mod advent;

fn main() {
    println!("{}", MERRY_CHRISTMAS);

    let day = std::env::args()
        .nth(1)
        .map(|s| s.parse::<usize>().expect("Invalid number"));

    let advent = Advent::new(input_folder());
    let report = match day {
        Some(number) => vec![advent.run_day(number)],
        _ => advent.run_all_days(),
    };

    println!("{}", display_table(&report));
}

fn input_folder() -> String {
    Path::new(file!())
        .parent()
        .unwrap()
        .join("input")
        .to_str()
        .unwrap()
        .into()
}

mod output {
    use cli_table::{Table, WithTitle};

    use crate::advent::{DayResult, ReportSlice};

    #[derive(Table)]
    struct ReportRow {
        #[table(title = "Day")]
        day: String,
        #[table(title = "Load Elapsed")]
        load_elapsed: String,
        #[table(title = "First Result")]
        first_result: String,
        #[table(title = "First Elapsed")]
        first_elapsed: String,
        #[table(title = "Second Result")]
        second_result: String,
        #[table(title = "Second Elapsed")]
        second_elapsed: String,
    }

    pub fn display_table(report: ReportSlice) -> String {
        report
            .iter()
            .map(build_row)
            .collect::<Vec<_>>()
            .with_title()
            .display()
            .unwrap()
            .to_string()
    }

    fn build_row(day: &DayResult) -> ReportRow {
        ReportRow {
            day: format!("{:02}", &day.number).to_string(),
            load_elapsed: format!("{:?}", &day.load_elapsed).to_string(),
            first_result: day.first_result.to_string(),
            first_elapsed: format!("{:?}", &day.first_elapsed).to_string(),
            second_result: day.second_result.to_string(),
            second_elapsed: format!("{:?}", &day.second_elapsed).to_string(),
        }
    }

    pub const MERRY_CHRISTMAS: &str = r"

                  ,---.  ,---.  ,---.  .-.   .-.                                 
                  |\    /| | .-'  | .-.\ | .-.\  \ \_/ )/                                 
                  |(\  / | | `-.  | `-'/ | `-'/   \   (_)                                 
                  (_)\/  | | .-'  |   (  |   (     ) (                                    
                  | \  / | |  `--.| |\ \ | |\ \    | |                                    
                  | |\/| | /( __.'|_| \)\|_| \)\  /(_|                                    
                  '-'  '-'(__)        (__)   (__)(__)                                     
         ,--,  .-. .-.,---.  .-. .-.   .---.  _______           .--.     .---. 
       .' .')  | | | || .-.\ | | | |  ( .-._)|__   __||\    /| / /\ \   ( .-._)
       |  |(_) | `-' || `-'/ | | | | (_) \     )| |   |(\  / |/ /__\ \ (_) \   
       \  \    | .-. ||   (  | | | | _  \ \   (_) |   (_)\/  ||  __  | _  \ \  
        \  `-. | | |)|| |\ \ | `-')|( `-'  )    | |   | \  / || |  |)|( `-'  ) 
         \____\/(  (_)|_| \)\`---(_) `----'     `-'   | |\/| ||_|  (_) `----'  
              (__)        (__)                        '-'  '-'                 

    ";
}
