use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        // do things
        let mut layers = input
            .trim()
            .split("")
            .skip(1)
            .collect::<Vec<_>>()
            .chunks(25 * 6)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
        layers.pop();
        layers
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let layers = munge_input!(input);

    let min_zero_layer = layers
        .into_iter()
        .min_by_key(|l| l.iter().filter(|x| **x == "0").count())
        .unwrap();

    eprintln!(
        "{:?}",
        min_zero_layer.iter().filter(|x| **x == "1").count()
            * min_zero_layer.iter().filter(|x| **x == "2").count()
    );

    Ok(())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let layers = munge_input!(input);

    let ans = layers
        .into_iter()
        .fold(vec![""; 25 * 6], |mut i, l| {
            for (i, &l) in i.iter_mut().zip(l.iter()) {
                match *i {
                    "" | "2" => *i = l,
                    _ => {}
                }
            }
            i
        })
        .into_iter()
        .map(|c| match c {
            "0" => ".",
            "1" => "X",
            _ => "_",
        })
        .collect::<Vec<_>>();

    ans.chunks(25)
        .for_each(|c| eprintln!("{}", c.to_vec().join("")));

    Ok(())
}
