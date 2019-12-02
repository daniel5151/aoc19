#![allow(clippy::unreadable_literal)]

pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod day1;
pub mod day2;

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let question = match args.get(1) {
        None => return Err("Must specify day and question (e.g: 1.1)".into()),
        Some(q) => q,
    };

    match question.as_str() {
        "1.1" => day1::q1(&args[2..])?,
        "1.2" => day1::q2(&args[2..])?,
        "2.1" => day2::q1(&args[2..])?,
        "2.2" => day2::q2(&args[2..])?,
        _ => return Err("Unknown question".into()),
    };

    Ok(())
}
