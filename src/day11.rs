use crate::prelude::*;

fn calc_path(input: String, seed: isize) -> DynResult<HashMap<(i32, i32), isize>> {
    let intcode = &mut Intcode::new(input)?;
    let input = &mut VecDeque::new();

    let mut tiles = HashMap::new();

    const DELTA: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut delta = 0; // facing up

    input.push_back(seed);
    let mut pos = (0, 0);

    while let Some(color) = intcode::run::until_output(intcode, input)? {
        // mark tile as painted with given color
        tiles.insert(pos, color);

        let rotation =
            intcode::run::until_output(intcode, input)?.ok_or("malfunctioning intcode program")?;

        match rotation {
            0 => delta = (delta + 3) % 4,
            1 => delta = (delta + 1) % 4,
            _ => return Err("malfunctioning intcode program".into()),
        }

        // move the robot forward
        pos.0 += DELTA[delta].0;
        pos.1 += DELTA[delta].1;

        // input the current tile color the robot is standing on
        input.push_back(*tiles.entry(pos).or_default())
    }

    Ok(tiles)
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let tiles = calc_path(input, 0)?;
    Ok(tiles.len())
}

pub fn q2(input: String, args: &[String]) -> DynResult<()> {
    let seed = match args.get(0).map(|x| x.as_str()) {
        Some("alt") => 0,
        Some(_) => return Err("did you mean `alt`?".into()),
        None => 1,
    };

    let tiles = calc_path(input, seed)?;

    // find output bounds
    let min_x = *tiles.iter().map(|((x, _), _)| x).min().unwrap();
    let min_y = *tiles.iter().map(|((_, y), _)| y).min().unwrap();
    let max_x = *tiles.iter().map(|((x, _), _)| x).max().unwrap();
    let max_y = *tiles.iter().map(|((_, y), _)| y).max().unwrap();

    let mut floor = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for ((x, y), color) in tiles {
        floor[(y - min_y) as usize][(x - min_x) as usize] = color;
    }

    for row in floor.into_iter().rev() {
        println!(
            "{}",
            row.into_iter()
                .map(|c| {
                    match c {
                        0 => " ".to_string(),
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
