use std::collections::{HashMap, HashSet, VecDeque};

use crate::DynResult;

///
/// ## --- Day 6: Universal Orbit Map ---
///
/// You've landed at the Universal Orbit Map facility on Mercury. Because
/// navigation in space often involves transferring between orbits, the orbit
/// maps here are useful for finding efficient routes between, for example, you
/// and Santa. You download a map of the local orbits (your puzzle input).
///
/// Except for the universal Center of Mass (`COM`), every object in space is in
//// orbit around <span title="What do you mean, Kerbal Space Program doesn't
/// have accurate orbital physics?">exactly one other object</span>.
///
/// An [orbit](https://en.wikipedia.org/wiki/Orbit) looks roughly like this:
///
///                       \
///                        \
///                         |
///                         |
///     AAA--> o            o <--BBB
///                         |
///                         |
///                        /
///                       /
///
/// In this diagram, the object `BBB` is in orbit around `AAA`. The path that
/// `BBB` takes around `AAA` (drawn with lines) is only partly shown. In the map
/// data, this orbital relationship is written `AAA)BBB`, which means "`BBB` is
/// in orbit around `AAA`".
///
/// Before you use your map data to plot a course, you need to make sure it
/// wasn't corrupted during the download. To verify maps, the Universal Orbit
/// Map facility uses _orbit count checksums_ - the total number of _direct
/// orbits_ (like the one shown above) and _indirect orbits_.
///
/// Whenever `A` orbits `B` and `B` orbits `C`, then `A` _indirectly orbits_
/// `C`. This chain can be any number of objects long: if `A` orbits `B`, `B`
/// orbits `C`, and `C` orbits `D`, then `A` indirectly orbits `D`.
///
/// For example, suppose you have the following map:
///
///     COM)B
///     B)C
///     C)D
///     D)E
///     E)F
///     B)G
///     G)H
///     D)I
///     E)J
///     J)K
///     K)L
///
/// Visually, the above map of orbits looks like this:
///
///             G - H       J - K - L
///            /           /
///     COM - B - C - D - E - F
///                    \
///                     I
///
/// In this visual representation, when two objects are connected by a line, the
/// one on the right directly orbits the one on the left.
///
/// Here, we can count the total number of orbits as follows:
///
/// * `D` directly orbits `C` and indirectly orbits `B` and `COM`, a total of
///   `3` orbits.
/// * `L` directly orbits `K` and indirectly orbits `J`, `E`, `D`, `C`, `B`, and
///   `COM`, a total of `7` orbits.
/// * `COM` orbits nothing.
///
/// The total number of direct and indirect orbits in this example is `_42_`.
///
/// _What is the total number of direct and indirect orbits_ in your map data?
pub fn q1(input: String, args: &[String]) -> DynResult<()> {
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

    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in pairs {
        orbits.entry(a).or_default().insert(b);
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
                if let Some(os) = orbits.get(planet) {
                    s.extend(os.iter().map(|o| (*o, depth + 1)));
                }
            }
            checksum
        }
    };

    println!("{}", checksum);

    Ok(())
}

fn checksum(orbits: &HashMap<&str, HashSet<&str>>, root: &str, d: usize) -> usize {
    d + orbits
        .get(root)
        .map(|o| o.iter().map(|p| checksum(orbits, p, d + 1)).sum())
        .unwrap_or(0)
}

/// ## --- Part Two ---
///
/// Now, you just need to figure out how many _orbital transfers_ you (`YOU`)
/// need to take to get to Santa (`SAN`).
///
/// You start at the object `YOU` are orbiting; your destination is the object
/// `SAN` is orbiting. An orbital transfer lets you move from any object to an
/// object orbiting or orbited by that object.
///
/// For example, suppose you have the following map:
///
///     COM)B
///     B)C
///     C)D
///     D)E
///     E)F
///     B)G
///     G)H
///     D)I
///     E)J
///     J)K
///     K)L
///     K)YOU
///     I)SAN
///
/// Visually, the above map of orbits looks like this:
///
///                               YOU
///                              /
///             G - H       J - K - L
///            /           /
///     COM - B - C - D - E - F
///                    \
///                     I - SAN
///
/// In this example, `YOU` are in orbit around `K`, and `SAN` is in orbit around
/// `I`. To move from `K` to `I`, a minimum of `4` orbital transfers are
/// required:
///
/// * `K` to `J`
/// * `J` to `E`
/// * `E` to `D`
/// * `D` to `I`
///
/// Afterward, the map of orbits looks like this:
///
///             G - H       J - K - L
///            /           /
///     COM - B - C - D - E - F
///                    \
///                     I - SAN
///                      \
///                       YOU
///
/// _What is the minimum number of orbital transfers required_ to move from the
/// object `YOU` are orbiting to the object `SAN` is orbiting? (Between the
/// objects they are orbiting - _not_ between `YOU` and `SAN`.)
pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
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

    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in pairs {
        edges.entry(a).or_default().insert(b);
        edges.entry(b).or_default().insert(a);
    }

    // basic BFS
    let mut q = VecDeque::new();
    q.push_back(("YOU", "", 0));
    while let Some((planet, parent, hops)) = q.pop_front() {
        if planet == "SAN" {
            println!("{}", hops - 2); // YOU and SAN don't count
            return Ok(());
        }

        q.extend(
            edges[planet]
                .iter()
                .filter(|p| **p != parent)
                .map(|p| (*p, planet, hops + 1)),
        );
    }

    println!("Couldn't find route from YOU to SAN");
    Ok(())
}
