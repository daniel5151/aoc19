use crate::prelude::*;

// Taking a page out of game development, and using a "structure of arrays"
// instead of an "array of structures"
macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;

        let mut pos = vec![Vec::new(); 3];
        let mut vel = vec![Vec::new(); 3];

        let lines = input.split('\n').map(|s| {
            s.chars()
                .filter(|&c| !"<>xyz=".contains(c))
                .collect::<String>()
        });

        for ln in lines {
            for (i, val) in ln.split(',').map(|s| s.trim().parse::<i32>()).enumerate() {
                pos[i].push(val?);
                vel[i].push(0);
            }
        }

        (pos, vel)
    }};
}

fn iter_moon_component(pos: &mut [i32], vel: &mut [i32]) {
    // update velocity based on position
    for ((ia, a), (ib, b)) in pos.iter().enumerate().tuple_combinations::<(_, _)>() {
        if a > b {
            vel[ia] -= 1;
            vel[ib] += 1;
        } else if a < b {
            vel[ia] += 1;
            vel[ib] -= 1;
        } else {
            // no change
        }
    }

    // update position based on velocity
    for (p, v) in pos.iter_mut().zip(vel.iter()) {
        *p += v
    }
}

pub fn q1(input: String, args: &[String]) -> DynResult<i32> {
    let iters = match args.get(0) {
        None => 1000,
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| "invalid num iters specified")?,
    };

    let (mut pos, mut vel) = munge_input!(input);

    for _ in 0..iters {
        for c in 0..3 {
            iter_moon_component(&mut pos[c], &mut vel[c])
        }
    }

    let count = pos.get(0).map(|v| v.len()).unwrap_or(0);

    let total_energy = (0..count)
        .map(|i| {
            let pos_e = pos[0][i].abs() + pos[1][i].abs() + pos[2][i].abs();
            let vel_e = vel[0][i].abs() + vel[1][i].abs() + vel[2][i].abs();
            pos_e * vel_e
        })
        .sum::<i32>();

    Ok(total_energy)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let (mut pos, mut vel) = munge_input!(input);

    let mut iters = vec![0; 3];

    for (c, iter) in iters.iter_mut().enumerate() {
        let mut past_states = HashSet::new();
        loop {
            let hash = (aoc::hash(&pos[c]), (aoc::hash(&vel[c])));
            if past_states.contains(&hash) {
                break;
            }
            past_states.insert(hash);

            iter_moon_component(&mut pos[c], &mut vel[c]);
            *iter += 1;
        }
    }

    let lcm = iters.into_iter().fold(1, |a, x| a.lcm(x));
    Ok(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const E1: &str = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const E2: &str = "
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn q1_e1() {
        let input = E1.trim();
        let output = q1(input.to_string(), &["10".to_string()]);
        assert_eq!(output.unwrap(), 179);
    }

    #[test]
    fn q1_e2() {
        let input = E2.trim();
        let output = q1(input.to_string(), &["100".to_string()]);
        assert_eq!(output.unwrap(), 1940);
    }

    #[test]
    fn q2_e1() {
        let input = E1.trim();
        let output = q2(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 2772);
    }

    #[test]
    fn q2_e2() {
        let input = E2.trim();
        let output = q2(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 4686774924);
    }
}
