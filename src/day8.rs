use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .chars()
            .collect::<Vec<_>>()
            .chunks(25 * 6)
            .map(|s| s.to_vec())
            .collect::<Vec<_>>()
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let layers = munge_input!(input);

    let count_char = |c| move |l: &Vec<char>| l.iter().filter(|x| **x == c).count();

    let l = layers.into_iter().min_by_key(count_char('0')).unwrap();

    Ok(count_char('1')(&l) * count_char('2')(&l))
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let layers = munge_input!(input);

    let ans = layers
        .into_iter()
        .fold(vec![' '; 25 * 6], |mut i, l| {
            for (i, l) in i.iter_mut().zip(l) {
                match *i {
                    ' ' | '2' => *i = l,
                    _ => {}
                }
            }
            i
        })
        // shuffle the chars around (for readability)
        .into_iter()
        .map(|c| match c {
            '0' => " ",
            '1' => "X",
            _ => "_",
        })
        .collect::<Vec<_>>()
        // chunk into lines and re-join to get the image
        .chunks(25)
        .map(|c| c.join(""))
        .collect::<Vec<_>>()
        .join("\n");

    eprintln!("{}", ans);

    Ok(())
}
