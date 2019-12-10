use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;

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

        if !wires.is_empty() {
            return Err("specified more than 2 wires".into());
        }

        (w1, w2)
    }};
}

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

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let (w1, w2) = munge_input!(input);

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
        Some(min_dist) => Ok(min_dist as usize),
    }
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let (w1, w2) = munge_input!(input);

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
        Some(min_steps) => Ok(min_steps),
    }
}
