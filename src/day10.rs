use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .split('\n')
            .enumerate()
            .flat_map(|(row, s)| {
                s.chars().enumerate().filter_map(move |(col, c)| {
                    // 'X' doesn't appear in the question input, but it's useful
                    // to annotate the station in the test-cases (for reasoning
                    // about angles and whatnot)
                    match c {
                        '#' | 'X' => Some(Ok((col as isize, row as isize))),
                        '.' => None,
                        _ => Some(Err("invalid char in input")),
                    }
                })
            })
            // use BTree for consistent ordering
            .collect::<Result<BTreeSet<(isize, isize)>, _>>()?
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<(usize, (isize, isize))> {
    let asteroids = munge_input!(input);

    let max = asteroids
        .iter()
        .map(|(sx, sy)| {
            asteroids
                .iter()
                .filter(|(x, y)| !(x == sx && y == sy))
                // translate to cartesian coordinates centered at (sx, sy)
                .map(|(x, y)| (x - sx, -(y - sy)))
                // calculate the angle between the planets
                .map(|(x, y)| (y as f32).atan2(x as f32))
                // f32 doesn't implement Ord, so move the decimal place forward
                // a bit, and cast to an integer
                .map(|angle| (angle * 100000.) as isize)
                // de-dupe angles
                .collect::<HashSet<_>>()
                .len()
        })
        .zip(asteroids.iter().copied())
        .max_by_key(|&(count, _)| count)
        .ok_or("input doesn't have any asteroids")?;

    Ok(max)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<isize> {
    let asteroids = munge_input!(input);

    let (sx, sy) = q1(input.clone(), &[])?.1;

    // <angle, <magniture, planet>>
    // using BTrees to auto-sort ascending by angle/magnitude
    let mut order: BTreeMap<usize, BTreeMap<isize, (isize, isize)>> = BTreeMap::new();

    for (x, y) in asteroids.into_iter() {
        // translate to cartesian coordinates centered at (sx, sy)
        let (cx, cy) = (x - sx, -(y - sy));

        use std::f32::consts::PI;
        let mut angle = (cy as f32).atan2(cx as f32);
        // rotate andgles so that 0 = pi/2
        angle -= PI / 2.;
        // normalize the angles between 0 and 2pi
        if angle < 0. {
            angle += 2. * PI;
        }
        // flip angle direction (to go clockwise)
        angle = -angle;

        let magnitude = cx.pow(2) + cy.pow(2);

        order
            .entry((angle * 100000.) as usize)
            .or_default()
            .insert(magnitude, (x, y));
    }

    let mut count = 0;
    let mut cleanup = Vec::new();
    while !order.is_empty() {
        // do a rotation
        for (&angle, asteroids) in order.iter_mut() {
            // get the nearest planet
            match asteroids.iter().next() {
                Some((&k, &(x, y))) => {
                    count += 1;
                    asteroids.remove(&k);
                    if count == 200 {
                        return Ok(x * 100 + y);
                    }
                }
                // no more asteroids at this angle, so we can avoid checking it
                // in subsequent rotations
                None => cleanup.push(angle),
            }
        }

        for a in cleanup.drain(..) {
            order.remove(&a);
        }
    }

    Err("less than 200 asteroids were destroyed".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "
.#..#
.....
#####
....#
...##";

        let output = q1(input.trim().to_string(), &[]);
        assert_eq!(output.unwrap(), (8, (3, 4)));
    }

    #[test]
    fn q1_e2() {
        let input = "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let output = q1(input.trim().to_string(), &[]);
        assert_eq!(output.unwrap(), (33, (5, 8)));
    }

    const BIG_INPUT: &str = r"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.###X######...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn q1_e3() {
        let input = BIG_INPUT;
        let output = q1(input.trim().to_string(), &[]);
        assert_eq!(output.unwrap(), (210, (11, 13)));
    }

    #[test]
    fn q2_e1() {
        let input = BIG_INPUT;
        let output = q2(input.trim().to_string(), &[]);
        assert_eq!(output.unwrap(), 802);
    }
}
