use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        let pairs = input
            .split('\n')
            .map(|s| {
                let mut c = s.splitn(2, ')');
                (c.next().unwrap(), c.next().unwrap_or(""))
            })
            .collect::<Vec<_>>();
        if pairs.iter().any(|(a, _)| a.is_empty()) {
            return Err("Malformed input".into());
        }
        pairs
    }};
}

pub fn q1(input: String, args: &[String]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in input {
        orbits.entry(a).or_default().insert(b);
        orbits.entry(b).or_default();
    }

    let checksum = match args.get(0).map(|s| s.as_str()) {
        Some("recursive") => checksum(&orbits, "COM", 0),
        Some(_) => return Err("invalid argument. did you mistype `recrusive`?".into()),
        None => {
            let mut s = Vec::new();
            s.push(("COM", 0));

            let mut checksum = 0;
            while let Some((planet, depth)) = s.pop() {
                checksum += depth;
                s.extend(orbits[planet].iter().map(|o| (*o, depth + 1)));
            }
            checksum
        }
    };

    Ok(checksum)
}

fn checksum(orbits: &HashMap<&str, HashSet<&str>>, root: &str, d: usize) -> usize {
    d + orbits
        .get(root)
        .map(|o| o.iter().map(|p| checksum(orbits, p, d + 1)).sum())
        .unwrap_or(0)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let input = munge_input!(input);

    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in input {
        orbits.entry(a).or_default().insert(b);
        orbits.entry(b).or_default().insert(a);
    }

    // basic BFS
    let mut q = VecDeque::new();
    q.push_back(("YOU", "", 0));
    while let Some((planet, parent, hops)) = q.pop_front() {
        if planet == "SAN" {
            return Ok(hops - 2); // YOU and SAN don't count
        }

        q.extend(
            orbits[planet]
                .iter()
                .filter(|p| **p != parent)
                .map(|p| (*p, planet, hops + 1)),
        );
    }

    Err("Couldn't find route from YOU to SAN".into())
}
