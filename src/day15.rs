use crate::prelude::*;

fn dir_to_delta(dir: i32) -> Option<(i32, i32)> {
    let delta = match dir {
        1 => (0, -1),
        2 => (0, 1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => return None,
    };
    Some(delta)
}

fn reverse_dir(dir: i32) -> Option<i32> {
    let dir = match dir {
        1 => 2, // N -> s
        2 => 1, // s -> N
        3 => 4, // W -> E
        4 => 3, // E -> W
        _ => return None,
    };
    Some(dir)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Floor,
    Oxygen,
    Queued,
}

fn visualize(room: &HashMap<(i32, i32), Tile>, pos: (i32, i32)) {
    // clear screen
    print!("\x1b[2J");
    print!("\x1b[{};{}H", 1, 1);

    if room.is_empty() {
        return;
    }

    // find output bounds
    let min_x = (*room.iter().map(|((x, _), _)| x).min().unwrap()).min(pos.0);
    let min_y = (*room.iter().map(|((_, y), _)| y).min().unwrap()).min(pos.1);
    let max_x = (*room.iter().map(|((x, _), _)| x).max().unwrap()).max(pos.0);
    let max_y = (*room.iter().map(|((_, y), _)| y).max().unwrap()).max(pos.1);

    let mut floor = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for ((x, y), color) in room {
        floor[(y - min_y) as usize][(x - min_x) as usize] = match *color {
            Tile::Wall => 1,
            Tile::Floor => 2,
            Tile::Oxygen => 3,
            Tile::Queued => 4,
        };
    }

    floor[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = 5;

    for row in floor.into_iter() {
        println!(
            "{}",
            row.into_iter()
                .map(|c| {
                    match c {
                        0 => " ",
                        1 => "#",
                        2 => ".",
                        3 => "O",
                        4 => "?",
                        5 => "D",
                        _ => unreachable!(),
                    }
                    .to_string()
                })
                .collect::<Vec<String>>()
                .join("")
        );
    }

    println!("{:?}", pos);
}

fn explore_map(input: String, with_vis: bool) -> DynResult<HashMap<(i32, i32), Tile>> {
    let intcode = &mut Intcode::new(input)?;
    let input = &mut VecDeque::new();

    let mut room: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut pos = (0, 0);

    // assumption: droid doesn't start on the oxygen
    room.insert((0, 0), Tile::Floor);

    let mut moves: Vec<i32> = Vec::new();
    loop {
        for dir in 1..=4 {
            let (dx, dy) = dir_to_delta(dir).unwrap();
            let next_pos = (pos.0 + dx, pos.1 + dy);

            room.entry(next_pos).or_insert_with(|| {
                // if the tile hasn't been explored, add it to the search queue.
                moves.push(reverse_dir(dir).unwrap());
                moves.push(dir);
                Tile::Queued
            });
        }

        if with_vis {
            std::thread::sleep(std::time::Duration::from_millis(8));
            visualize(&room, pos);
        }

        let dir = match moves.pop() {
            Some(dir) => dir,
            None => break,
        };

        input.push_back(dir as isize);
        let output = intcode::run::until_output(intcode, input)?.ok_or("malfunctioning intcode")?;
        let output = match output {
            0 => Tile::Wall,
            1 => Tile::Floor,
            2 => Tile::Oxygen,
            _ => return Err("malfunctioning intcode".into()),
        };

        // update room with new info
        let (dx, dy) = dir_to_delta(dir).unwrap();
        let next_pos = (pos.0 + dx, pos.1 + dy);
        room.insert(next_pos, output);

        // move robot / adjust robot search path
        match output {
            Tile::Wall => {
                moves.pop().ok_or("bruh")?;
            }
            Tile::Floor | Tile::Oxygen => pos = next_pos,
            _ => unreachable!(),
        }
    }

    Ok(room)
}

pub fn q1(input: String, args: &[String]) -> DynResult<usize> {
    let with_vis = match args.get(0).map(|s| s.as_str()) {
        None => false,
        Some("vis") => true,
        Some(_) => return Err("invalid arg (did you mean to type `vis`?)".into()),
    };

    let room = explore_map(input, with_vis)?;

    // BFS to find shortest path
    let mut q = VecDeque::new();
    let mut v = HashSet::new();
    q.push_back(((0, 0), 0));

    while let Some((pos, dist)) = q.pop_front() {
        if v.contains(&pos) {
            continue;
        }
        v.insert(pos);

        match room.get(&pos) {
            Some(Tile::Oxygen) => {
                return Ok(dist);
            }
            Some(Tile::Wall) | None => continue,
            Some(Tile::Floor) => {}
            // map should be fully explored
            Some(Tile::Queued) => unreachable!(),
        }

        for (dx, dy) in (1..=4).map(|d| dir_to_delta(d).unwrap()) {
            let next_pos = (pos.0 + dx, pos.1 + dy);
            q.push_back((next_pos, dist + 1))
        }
    }

    Err("could not find shortest path".into())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let with_vis = false;

    let mut room = explore_map(input, with_vis)?;

    let (start_pos, _) = room
        .iter()
        .find(|(_, tile)| **tile == Tile::Oxygen)
        .ok_or("didn't find any oxygen in the map")?;

    // BFS to find
    let mut max_time = 0;
    let mut edge = VecDeque::new();
    edge.push_back((*start_pos, 0));

    while let Some((pos, time)) = edge.pop_front() {
        max_time = max_time.max(time);

        for (dx, dy) in (1..=4).map(|d| dir_to_delta(d).unwrap()) {
            let next_pos = (pos.0 + dx, pos.1 + dy);

            match room.get(&next_pos) {
                Some(Tile::Oxygen) | Some(Tile::Wall) | None => continue,
                Some(Tile::Floor) => {
                    room.insert(next_pos, Tile::Oxygen);
                    edge.push_back((next_pos, time + 1))
                }
                // map should be fully explored
                Some(Tile::Queued) => unreachable!(),
            }
        }
    }

    Ok(max_time)
}
