#![allow(clippy::unreadable_literal)]
#![allow(clippy::type_complexity)]

/// Catch-all error type (works with anything that implements std::error::Error)
pub type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod prelude {
    // useful stdlib stuff
    pub use std::collections::*;
    pub use std::io::prelude::*;

    // useful external libraries
    pub use itertools::Itertools;

    // useful AOC things
    pub use crate::DynResult;
    pub use intcode::{self, Intcode};

    pub trait GcdLcm {
        /// Find the Greatest Common Divisor between two integers
        fn gcd(self, other: Self) -> Self;
        /// Find the Lowest Common Multiple between two integers
        fn lcm(self, other: Self) -> Self;
    }

    macro_rules! gcdlcm_impl {
        ($($type:ty),*) => ($(
            impl GcdLcm for $type {
                fn gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = if self > other {
                        (self, other)
                    } else {
                        (other, self)
                    };

                    while b != 0 {
                        let r = a % b;
                        a = b;
                        b = r;
                    }

                    a
                }

                fn lcm(self, other: Self) -> Self {
                    self * other / self.gcd(other)
                }
            }
        )*)
    }

    gcdlcm_impl! { u8, u16, u32, u64, u128, usize }

    pub mod aoc {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        pub fn hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
    }
}

// Utulity macro to make adding new days a breeze
macro_rules! days {
    ($($day:ident),* $(,)*) => {
        $(mod $day;)*

        fn route_day(day: &str, question: &str, input: String, other_args: &[String]) -> DynResult<()> {
            let day = format!("day{}", day);

            match day.as_str() {
                $(stringify!($day) => match question {
                    "1" => println!("Answer: {:?}", $day::q1(input, other_args)?),
                    "2" => println!("Answer: {:?}", $day::q2(input, other_args)?),
                    _ => return Err("Unknown question".into()),
                })*
                _ => return Err("Unknown day".into()),
            }

            Ok(())
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
    day7,
    day8,
    day9,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
}

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let (day, question) = match (args.get(1), args.get(2)) {
        (None, _) | (_, None) => return Err("Must specify day and question (e.g: 3 1)".into()),
        (Some(d), Some(q)) => (d.as_str(), q.as_str()),
    };

    let input_path = format!("./inputs/{}.txt", day);
    let input_path = std::path::Path::new(&input_path);

    let mut input = std::fs::read_to_string(input_path)
        .map_err(|e| format!("Could not open {}: {}", input_path.to_string_lossy(), e))?;
    input.truncate(input.trim_end().len());

    route_day(day, question, input, &args[3..])
}
