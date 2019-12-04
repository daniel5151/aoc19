use crate::DynResult;

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

/// ## --- Day 4: Secure Container ---
///
/// You arrive at the Venus fuel depot only to discover it's protected by a
/// password. The Elves had written the password on a sticky note, but someone
/// <span title="Look on the bright side - isn't it more secure if nobody knows
/// the password?">threw it out</span>.
///
/// However, they do remember a few key facts about the password:
///
/// * It is a six-digit number.
/// * The value is within the range given in your puzzle input.
/// * Two adjacent digits are the same (like `22` in `1_22_345`).
/// * Going from left to right, the digits _never decrease_; they only ever
///   increase or stay the same (like `111123` or `135679`).
///
/// Other than the range rule, the following are true:
///
/// * `111111` meets these criteria (double `11`, never decreases).
/// * `2234_50_` does not meet these criteria (decreasing pair of digits `50`).
/// * `123789` does not meet these criteria (no double).
///
/// _How many different passwords_ within the range given in your puzzle input
/// meet these criteria?
pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let mut input = input.split('-');
    let start = input.next().unwrap().parse::<usize>()?;
    let end = input.next().unwrap().parse::<usize>()?;

    let ans = (start..=end)
        .map(digits)
        .filter(|d| has_pair(&d))
        .filter(|d| ascending(&d))
        .count();

    eprintln!("{:?}", ans);

    Ok(())
}

/// ## --- Part Two ---
///
/// An Elf just remembered one more important detail: the two adjacent matching
/// digits _are not part of a larger group of matching digits_.
///
/// Given this additional criterion, but still ignoring the range rule, the
/// following are now true:
///
/// * `112233` meets these criteria because the digits never decrease and all
///   repeated digits are exactly two digits long.
/// * `123_444_` no longer meets the criteria (the repeated `44` is part of a
///   larger group of `444`).
/// * `111122` meets the criteria (even though `1` is repeated more than twice,
///    it still contains a double `22`).
///
/// _How many different passwords_ within the range given in your puzzle input
/// meet all of the criteria?
pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let mut input = input.split('-');
    let start = input.next().unwrap().parse::<usize>()?;
    let end = input.next().unwrap().parse::<usize>()?;

    let ans = (start..=end)
        .map(digits)
        .filter(|d| has_pair(&d))
        .filter(|d| ascending(&d))
        .filter(|d| has_run2(&d))
        .count();

    eprintln!("{:?}", ans);

    Ok(())
}
