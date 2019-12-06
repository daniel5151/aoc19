#![allow(clippy::unreadable_literal)]

use std::path::Path;

pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod intcode;

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let (day, question) = match (args.get(1), args.get(2)) {
        (None, _) | (_, None) => return Err("Must specify day and question (e.g: 3 1)".into()),
        (Some(d), Some(q)) => (d.as_str(), q.as_str()),
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = Path::new(&input_path);

    let input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Could not open {}: {}", input_path.to_string_lossy(), e))?;

    macro_rules! day {
        ($day:ident) => {
            match question {
                "1" => $day::q1(input, &args[2..])?,
                "2" => $day::q2(input, &args[2..])?,
                _ => return Err("Unknown question".into()),
            }
        };
    }

    match day {
        "1" => day!(day1),
        "2" => day!(day2),
        "3" => day!(day3),
        "4" => day!(day4),
        "5" => day!(day5),
        _ => return Err("Unknown day".into()),
    };

    Ok(())
}
