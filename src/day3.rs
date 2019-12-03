use std::collections::HashMap;
use std::collections::HashSet;

use crate::DynResult;

struct PathChunk {
    pub dir: Dir,
    pub dist: usize,
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_delta(self) -> (i32, i32) {
        use self::Dir::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

impl std::str::FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Dir, ()> {
        use self::Dir::*;
        let res = match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => return Err(()),
        };
        Ok(res)
    }
}

impl std::str::FromStr for PathChunk {
    type Err = ();
    fn from_str(s: &str) -> Result<PathChunk, ()> {
        Ok(PathChunk {
            dir: s[..1].parse()?,
            dist: s[1..].parse().map_err(drop)?,
        })
    }
}

fn parse_input(input: String) -> DynResult<(Vec<PathChunk>, Vec<PathChunk>)> {
    let mut wires = input
        .split('\n')
        .map(|s| {
            s.split(',')
                .map(|s| s.parse::<PathChunk>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<std::result::Result<Vec<_>, _>>()
        .map_err(|_| "failed to parse input")?;

    let w2 = wires.pop().ok_or_else(|| "second wire path unspecified")?;
    let w1 = wires.pop().ok_or_else(|| "first wire path unspecified")?;

    assert!(wires.is_empty());

    Ok((w1, w2))
}

/// ## --- Day 3: Crossed Wires ---
///
/// The gravity assist was successful, and you're well on your way to the Venus
/// refuelling station. During the rush back on Earth, the fuel management
/// system wasn't completely installed, so that's next on the priority list.
///
/// Opening the front panel reveals a jumble of wires. Specifically, _two wires_
/// are connected to a central port and extend outward on a grid. You trace the
/// path each wire takes as it leaves the central port, one wire per line of
/// text (your puzzle input).
///
/// The wires <span title="A jumble of twisty little wires, all alike.">
/// twist and turn</span>, but the two wires occasionally cross paths. To fix
/// the circuit, you need to _find the intersection point closest to the central
/// port_. Because the wires are on a grid, use the
/// [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) for
/// this measurement. While the wires do technically cross right at the central
/// port where they both start, this point does not count, nor does a wire count
/// as crossing with itself.
///
/// For example, if the first wire's path is `R8,U5,L5,D3`, then starting from
/// the central port (`o`), it goes right `8`, up `5`, left `5`, and finally
/// down `3`:
///
///     ...........
///     ...........
///     ...........
///     ....+----+.
///     ....|....|.
///     ....|....|.
///     ....|....|.
///     .........|.
///     .o-------+.
///     ...........
///
/// Then, if the second wire's path is `U7,R6,D4,L4`, it goes up `7`, right `6`,
/// down `4`, and left `4`:
///
///     ...........
///     .+-----+...
///     .|.....|...
///     .|..+--X-+.
///     .|..|..|.|.
///     .|.-X--+.|.
///     .|..|....|.
///     .|.......|.
///     .o-------+.
///     ...........
///
/// These wires cross at two locations (marked `X`), but the lower-left one is
/// closer to the central port: its distance is `3 + 3 = 6`.
///
/// Here are a few more examples:
///
/// * `R75,D30,R83,U83,L12,D49,R71,U7,L72 U62,R66,U55,R34,D71,R55,D58,R83` =
///   distance `159`
/// * `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
///   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7` = distance `135`
///
/// _What is the Manhattan distance_ from the central port to the closest
/// intersection?
pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let (w1, w2) = parse_input(input)?;

    // let's just brute force it wheeee

    let mut w1_points: HashSet<(i32, i32)> = HashSet::new();

    let mut cur_p = (0, 0);
    for PathChunk { dir, dist } in w1 {
        let (dx, dy) = dir.to_delta();
        for _ in 0..dist {
            cur_p = (cur_p.0 + dx, cur_p.1 + dy);
            w1_points.insert(cur_p.clone());
        }
    }

    let mut min_dist = None;
    let mut cur_p = (0, 0);
    for PathChunk { dir, dist } in w2 {
        let (dx, dy) = dir.to_delta();
        for _ in 0..dist {
            cur_p = (cur_p.0 + dx, cur_p.1 + dy);

            if w1_points.contains(&cur_p) {
                let cur_dist = cur_p.0.abs() + cur_p.1.abs();
                min_dist = match min_dist {
                    None => Some(cur_dist),
                    Some(min_dist) => Some(std::cmp::min(min_dist, cur_dist)),
                };
            }
        }
    }

    match min_dist {
        None => Err("Two wires didn't intersect".into()),
        Some(min_dist) => {
            println!("{:?}", min_dist);
            Ok(())
        }
    }
}

/// ## --- Part Two ---
///
/// It turns out that this circuit is very timing-sensitive; you actually need
/// to _minimize the signal delay_.
///
/// To do this, calculate the _number of steps_ each wire takes to reach each
/// intersection; choose the intersection where the _sum of both wires' steps_
/// is lowest. If a wire visits a position on the grid multiple times, use the
/// steps value from the _first_ time it visits that position when calculating
/// the total value of a specific intersection.
///
/// The number of steps a wire takes is the total number of grid squares the
/// wire has entered to get to that location, including the intersection being
/// considered. Again consider the example from above:
///
///     ...........
///     .+-----+...
///     .|.....|...
///     .|..+--X-+.
///     .|..|..|.|.
///     .|.-X--+.|.
///     .|..|....|.
///     .|.......|.
///     .o-------+.
///     ...........
///
/// In the above example, the intersection closest to the central port is
/// reached after `8+5+5+2 = _20_` steps by the first wire and `7+6+4+3 = _20_`
/// steps by the second wire for a total of `20+20 = _40_` steps.
///
/// However, the top-right intersection is better: the first wire takes only
/// `8+5+2 = _15_` and the second wire takes only `7+6+2 = _15_`, a total of
/// `15+15 = _30_` steps.
///
/// Here are the best steps for the extra examples from above:
///
/// * `R75,D30,R83,U83,L12,D49,R71,U7,L72 U62,R66,U55,R34,D71,R55,D58,R83` =
///   `610` steps
/// * `R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
///   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7` = `410` steps
///
/// _What is the fewest combined steps the wires must take to reach an
/// intersection?_
pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let (w1, w2) = parse_input(input)?;

    // ((x, y), step)
    let mut w1_points: HashMap<(i32, i32), usize> = HashMap::new();

    let mut cur_p = (0, 0);
    let mut cur_step = 0;
    for PathChunk { dir, dist } in w1 {
        let (dx, dy) = dir.to_delta();
        for _ in 0..dist {
            cur_p = (cur_p.0 + dx, cur_p.1 + dy);
            cur_step += 1;
            w1_points.insert(cur_p, cur_step);
        }
    }

    let mut min_steps = None;

    let mut cur_p = (0, 0);
    let mut cur_step = 0;
    for PathChunk { dir, dist } in w2 {
        let (dx, dy) = dir.to_delta();
        for _ in 0..dist {
            cur_p = (cur_p.0 + dx, cur_p.1 + dy);
            cur_step += 1;

            if let Some(w1_step) = w1_points.get(&cur_p) {
                min_steps = match min_steps {
                    None => Some(w1_step + cur_step),
                    Some(min_steps) => Some(std::cmp::min(min_steps, w1_step + cur_step)),
                };
            }
        }
    }

    match min_steps {
        None => Err("Two wires didn't intersect".into()),
        Some(min_steps) => {
            println!("{:?}", min_steps);
            Ok(())
        }
    }
}
