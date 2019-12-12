use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input
            .split('\n')
            .map(|s| {
                s.chars()
                    .filter(|&c| !"<>xyz=".contains(c))
                    .collect::<String>()
            })
            .map(|s| {
                s.split(',')
                    .map(|s| s.trim().parse::<i32>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<i32>>, _>>()?
    }};
}

fn iter_moons_by_component(pos: &mut Vec<Vec<i32>>, vel: &mut Vec<Vec<i32>>, d: usize) {
    // update velocity based on position
    for ((ia, a), (ib, b)) in pos.iter().enumerate().tuple_combinations::<(_, _)>() {
        if a[d] > b[d] {
            vel[ia][d] -= 1;
            vel[ib][d] += 1;
        } else if a[d] < b[d] {
            vel[ia][d] += 1;
            vel[ib][d] -= 1;
        } else {
            // no change
        }
    }

    // update position based on velocity
    for (p, v) in pos.iter_mut().zip(vel.iter()) {
        p[d] += v[d]
    }
}

fn iter_moons(pos: &mut Vec<Vec<i32>>, vel: &mut Vec<Vec<i32>>) {
    for d in 0..3 {
        iter_moons_by_component(pos, vel, d)
    }
}

pub fn q1(input: String, args: &[String]) -> DynResult<i32> {
    let iters = match args.get(0) {
        None => 1000,
        Some(v) => v
            .parse::<usize>()
            .map_err(|_| "invalid num iters specified")?,
    };

    let mut pos = munge_input!(input);
    let mut vel = vec![vec![0, 0, 0]; pos.len()];

    for _step in 0..iters {
        iter_moons(&mut pos, &mut vel);
    }

    let energy = |p: Vec<i32>| p.into_iter().map(|v| v.abs()).sum::<i32>();
    let total_energy = pos
        .into_iter()
        .zip(vel.into_iter())
        .map(|(pos, vel)| energy(pos) * energy(vel))
        .sum::<i32>();

    Ok(total_energy)
}

fn calc_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let mut pos = munge_input!(input);
    let mut vel = vec![vec![0, 0, 0]; pos.len()];

    let mut component_cycles = Vec::new();

    for d in 0..3 {
        let mut past_states = HashSet::new();

        let mut iter = 0;
        loop {
            let hash = (calc_hash(&pos), (calc_hash(&vel)));
            if past_states.contains(&hash) {
                break;
            }
            past_states.insert(hash);

            iter_moons_by_component(&mut pos, &mut vel, d);
            iter += 1;
        }

        component_cycles.push(iter as usize)
    }

    let lcm = component_cycles.into_iter().fold(1, |a, x| a.lcm(x));
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
