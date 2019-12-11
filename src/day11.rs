use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;

        input.split('\n')
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let mut intcode = Intcode::new(input)?;

    let mut input = VecDeque::new();

    let mut tiles = HashMap::new();

    let mut pos = (0, 0);

    let mut delta = (0, 1);

    input.push_back(0);
    loop {
        let color = intcode::run::until_output(&mut intcode, &mut input)?;
        let color = match color {
            Some(c) => c,
            None => break,
        };
        let rot = intcode::run::until_output(&mut intcode, &mut input)?.unwrap();

        tiles.insert(pos, color);

        match rot {
            0 => {
                delta = match delta {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                }
            }
            1 => {
                delta = match delta {
                    (0, 1) => (-1, 0),
                    (1, 0) => (0, 1),
                    (0, -1) => (1, 0),
                    (-1, 0) => (0, -1),
                    _ => unreachable!(),
                }
            }
            _ => unimplemented!(),
        }

        pos.0 += delta.0;
        pos.1 += delta.1;

        input.push_back(*tiles.entry(pos).or_default())
    }

    eprintln!("{:?}", tiles);

    Ok(tiles.len())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let mut intcode = Intcode::new(input)?;

    let mut input = VecDeque::new();

    let mut tiles = HashMap::new();

    let mut pos = (0, 0);

    let mut delta: (i32, i32) = (0, 1);

    input.push_back(1);
    loop {
        let color = intcode::run::until_output(&mut intcode, &mut input)?;
        let color = match color {
            Some(c) => c,
            None => break,
        };
        let rot = intcode::run::until_output(&mut intcode, &mut input)?.unwrap();

        tiles.insert(pos, color);

        match rot {
            0 => {
                delta = match delta {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => unreachable!(),
                }
            }
            1 => {
                delta = match delta {
                    (0, 1) => (-1, 0),
                    (1, 0) => (0, 1),
                    (0, -1) => (1, 0),
                    (-1, 0) => (0, -1),
                    _ => unreachable!(),
                }
            }
            _ => unimplemented!(),
        }

        pos.0 += delta.0;
        pos.1 += delta.1;

        input.push_back(*tiles.entry(pos).or_default())
    }

    // find bounds
    let min_x = *tiles.iter().map(|((x, y), _)| x).min().unwrap();
    let min_y = *tiles.iter().map(|((x, y), _)| y).min().unwrap();
    let max_x = *tiles.iter().map(|((x, y), _)| x).max().unwrap();
    let max_y = *tiles.iter().map(|((x, y), _)| y).max().unwrap();

    let mut out = vec![vec![0; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];

    for ((x, y), color) in tiles {
        out[(x - min_x) as usize][(y - min_y) as usize] = color;
    }

    for row in out {
        println!(
            "{}",
            row.into_iter()
                .map(|c| {
                    match c {
                        0 => ".".to_string(),
                        1 => "X".to_string(),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn q1_e1() {
    //     let input = "...";
    //     let output = q1(input.to_string(), &[]);
    //     assert_eq!(output.unwrap(), ());
    // }
}
