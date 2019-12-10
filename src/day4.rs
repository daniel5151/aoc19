use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        let mut input = input.split('-');
        let start = input.next().unwrap().parse::<usize>()?;
        let end = input.next().unwrap().parse::<usize>()?;
        (start, end)
    }};
}

fn digits(val: usize) -> Vec<char> {
    val.to_string().chars().collect()
}

fn has_pair(digits: &[char]) -> bool {
    digits.windows(2).any(|p| p[0] == p[1])
}

fn ascending(digits: &[char]) -> bool {
    digits.windows(2).all(|p| p[1] >= p[0])
}

fn has_run2(digits: &[char]) -> bool {
    let mut run = 1;
    for p in digits.windows(2) {
        if p[0] == p[1] {
            run += 1
        } else {
            if run == 2 {
                break;
            }
            run = 1
        }
    }
    run == 2
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let (start, end) = munge_input!(input);

    let ans = (start..=end)
        .map(digits)
        .filter(|d| has_pair(&d))
        .filter(|d| ascending(&d))
        .count();

    Ok(ans)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let (start, end) = munge_input!(input);

    let ans = (start..=end)
        .map(digits)
        .filter(|d| has_pair(&d))
        .filter(|d| ascending(&d))
        .filter(|d| has_run2(&d))
        .count();

    Ok(ans)
}
