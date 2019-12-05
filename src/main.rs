#![allow(clippy::unreadable_literal)]

use std::path::Path;

pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let (day, question) = match args.get(1) {
        None => return Err("Must specify day and question (e.g: 1.1)".into()),
        Some(q) => {
            let mut q = q.split('.');
            (
                q.next().ok_or_else(|| "Must specify day")?,
                q.next().ok_or_else(|| "Must specify question")?,
            )
        }
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = Path::new(&input_path);

    let input = if !input_path.exists() {
        std::fs::create_dir_all("./inputs")?;

        let cookie = std::fs::read_to_string("./cookie.txt")
            .map_err(|e| format!("Could not find 'cookie.txt': {:?}", e))?;

        eprintln!("Downloading input...");
        let mut input = reqwest::blocking::Client::new()
            .get(&format!("https://adventofcode.com/2019/day/{}/input", day))
            .header(reqwest::header::COOKIE, cookie)
            .send()?
            .text()?;
        std::fs::write(input_path, &input.trim())?;
        input.truncate(input.len() - 1); // trim without reallocating
        input
    } else {
        std::fs::read_to_string(input_path)?
    };

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
