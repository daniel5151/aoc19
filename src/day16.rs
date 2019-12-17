use crate::prelude::*;

use std::iter;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .chars()
            .map(|x| x.to_digit(10).map(|x| x as i32))
            .collect::<Option<Vec<_>>>()
            .ok_or("invalid input")?
    }};
}

fn pattern(i: usize) -> impl Iterator<Item = i32> {
    (iter::repeat(0).take(i))
        .chain(iter::repeat(1).take(i))
        .chain(iter::repeat(0).take(i))
        .chain(iter::repeat(-1).take(i))
        .cycle()
        .skip(1)
}

fn fft(input: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();
    for i in 0..input.len() {
        let n = input
            .iter()
            .zip(pattern(i + 1))
            .map(|(a, b)| a * b)
            .sum::<i32>();
        out.push(n.abs() % 10)
    }
    out
}

pub fn q1(input: String, _args: &[String]) -> DynResult<String> {
    let mut input = munge_input!(input);

    for _ in 0..100 {
        input = fft(input);
    }

    let ans = input
        .into_iter()
        .take(8)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");

    Ok(ans)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<String> {
    let input = munge_input!(input);
    let in_len = input.len();

    let input = input
        .into_iter()
        .cycle()
        .take(in_len * 10000)
        .collect::<Vec<i32>>();

    let input = fft(input);

    // lol no this ain't it chief

    // there's definately a trick here, probably something to do with linear algebra
    // / matrix multiplication. unfortunately, I haven't used it in a while, and
    // nothing is jumping out at me...

    eprintln!("{:?}", input);

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "80871224585914546619083218645595";
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), "24176176".to_string());
    }
}
