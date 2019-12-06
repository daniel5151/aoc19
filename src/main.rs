#![allow(clippy::unreadable_literal)]

use std::path::Path;

pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! days {
    ($($day:ident),* $(,)*) => {
        $(mod $day;)*

        fn route_day(day: &str, question: &str, input: String, other_args: &[String]) -> DynResult<()> {
            let day = format!("day{}", day);

            match day.as_str() {
                $(stringify!($day) => match question {
                    "1" => $day::q1(input, other_args),
                    "2" => $day::q2(input, other_args),
                    _ => Err("Unknown question".into()),
                })*
                _ => Err("Unknown day".into()),
            }
        }
    };
}

days! {
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
}

mod intcode;

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let (day, question) = match (args.get(1), args.get(2)) {
        (None, _) | (_, None) => return Err("Must specify day and question (e.g: 3 1)".into()),
        (Some(d), Some(q)) => (d.as_str(), q.as_str()),
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = Path::new(&input_path);

    let mut input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Could not open {}: {}", input_path.to_string_lossy(), e))?;
    input.truncate(input.trim_end().len());

    route_day(day, question, input, &args[3..])
}
