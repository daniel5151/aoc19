use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;

        let rows = input.split('\n').count();
        let cols = input.split('\n').next().unwrap().as_bytes().len();
        let asteroids = input
            .split('\n')
            .enumerate()
            .flat_map(|(row, s)| {
                s.as_bytes().iter().enumerate().filter_map(move |(col, c)| {
                    if *c == b'#' || *c == b'X' {
                        Some((col as isize, row as isize))
                    } else {
                        None
                    }
                })
            })
            .collect::<BTreeSet<(isize, isize)>>();
        ((rows, cols), asteroids)
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<(usize, (isize, isize))> {
    let ((rows, cols), asteroids) = munge_input!(input);

    let mut max = (0, (0, 0));
    for (x, y) in asteroids.iter().cloned() {
        let mut count = 0;

        let mut dirs = HashSet::new();
        for dx in 0..cols {
            for dy in 0..rows {
                let mut vx = x - dx as isize;
                let mut vy = y - dy as isize;

                if (vx, vy) == (0, 0) {
                    continue;
                }

                let gcd = (vx.abs() as usize).gcd(vy.abs() as usize) as isize;
                if gcd != 0 {
                    vx /= gcd;
                    vy /= gcd;
                } else {
                    if vx != 0 {
                        vx /= vx.abs()
                    };
                    if vy != 0 {
                        vy /= vy.abs()
                    };
                }

                if dirs.contains(&(vx, vy)) {
                    continue;
                } else {
                    dirs.insert((vx, vy));
                }

                let mut testx = x - vx;
                let mut testy = y - vy;

                while (0..rows as isize).contains(&testy) && (0..cols as isize).contains(&testx) {
                    if asteroids.contains(&(testx, testy)) {
                        count += 1;
                        break;
                    } else {
                        testx -= vx;
                        testy -= vy;
                    }
                }
            }
        }

        if max.0 < count {
            max = (count, (x, y))
        }
    }

    Ok(max)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<isize> {
    let (_, asteroids) = munge_input!(input);

    let (sx, sy) = q1(input.clone(), &[])?.1;

    // convert coordinates to cartesian plane centered on the station
    let asteroids = asteroids
        .into_iter()
        .map(|(x, y)| (x - sx, -(y - sy)))
        .collect::<BTreeSet<_>>();

    let mut order: BTreeMap<usize, BTreeMap<isize, (isize, isize)>> = BTreeMap::new();

    for (x, y) in asteroids {
        use std::f32::consts::PI;
        let mut angle = (y as f32).atan2(x as f32);
        if angle < 0. {
            angle += 2. * PI;
        }
        // order angles so that 0 = pi/2
        angle -= PI / 2.;
        if angle < 0. {
            angle += 2. * PI;
        }
        // and make the direction counter-clockwise
        angle = -angle;

        let magnitude = x.pow(2) + y.pow(2);

        order
            .entry((angle * 10000000.) as usize)
            .or_default()
            .insert(magnitude, (x + sx, -(y - sy)));
    }

    let mut count = 0;
    while !order.is_empty() {
        let mut cleanup = Vec::new();

        for (&angle, asteroids) in order.iter_mut() {
            match asteroids.iter().next() {
                Some((&k, (x, y))) => {
                    count += 1;
                    if count == 200 {
                        return Ok(x * 100 + y);
                    }
                    asteroids.remove(&k);
                }
                None => cleanup.push(angle),
            }
        }
        for a in cleanup {
            order.remove(&a);
        }
    }

    println!("{:?}", order);

    Err("not enough asteroids destroyed".into())
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
...##
    "
        .trim();
        let output = q1(input.to_string(), &[]);
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
.#....####
    "
        .trim();
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), (33, (5, 8)));
    }

    const BIG_INPUT: &str = "
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
        let input = BIG_INPUT.trim();
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), (210, (11, 13)));
    }

    #[test]
    fn q2_e1() {
        let input = BIG_INPUT.trim();
        let output = q2(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 802);
    }
}

pub trait Gcd {
    /// Determine [greatest common divisor](https://en.wikipedia.org/wiki/Greatest_common_divisor)
    /// using the [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm)
    ///
    /// # Examples
    ///
    /// ```
    /// use gcd::Gcd;
    ///
    /// assert_eq!(0, 0u8.gcd(0));
    /// assert_eq!(10, 10u8.gcd(0));
    /// assert_eq!(10, 0u8.gcd(10));
    /// assert_eq!(10, 10u8.gcd(20));
    /// assert_eq!(44, 2024u32.gcd(748));
    /// ```
    fn gcd(self, other: Self) -> Self;
}

macro_rules! gcd_impl {
    ($($t:ty),*) => ($(
        impl Gcd for $t {
            fn gcd(self, other: Self) -> Self {
                // variable names based off Euclidean divison equation: a = b · q + r
                let (mut a, mut b) = if self > other {
                    (self, other)
                } else {
                    (other, self)
                };

                while b != 0 {
                    let r = a % b;
                    a = b;
                    b = r;
                }

                a
            }
        }
    )*)
}

gcd_impl! { u8, u16, u32, u64, u128, usize }
