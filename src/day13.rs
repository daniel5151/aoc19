use crate::prelude::*;

struct GameState {
    tiles: HashMap<(isize, isize), isize>,
    score: isize,
}

fn run_game(intcode: &mut Intcode, delay: u64) -> DynResult<GameState> {
    let input = &mut VecDeque::new();

    let mut tiles = HashMap::new();
    let mut score = 0;

    // clear screen
    print!("\x1b[2J");

    // AI
    let mut ball_x = 0;
    let mut paddle_x = 0;

    loop {
        // AI
        let cmd = if ball_x < paddle_x {
            -1
        } else if ball_x > paddle_x {
            1
        } else {
            0
        };

        input.clear();
        input.push_back(cmd);

        let x = intcode::run::until_output(intcode, input)?;
        let x = match x {
            Some(x) => x,
            // game over
            None => {
                print!("\x1b[{};{}H", 30, 1);
                return Ok(GameState { tiles, score });
            }
        };

        let y =
            intcode::run::until_output(intcode, input)?.ok_or("malfunctioning intcode program")?;
        let kind =
            intcode::run::until_output(intcode, input)?.ok_or("malfunctioning intcode program")?;

        if (x, y) == (-1, 0) {
            score = kind;

            // move cursor
            print!("\x1b[{};{}H", 1, 1);
            print!("Score: {:<10}", score);
        } else {
            tiles.insert((x, y), kind);

            // AI
            match kind {
                3 => paddle_x = x,
                4 => ball_x = x,
                _ => {}
            }

            // move cursor
            print!("\x1b[{};{}H", 1 + y + 1, 1 + x * 3);
            print!(
                "{}",
                match kind {
                    0 => "   ".to_string(),
                    1 => "...".to_string(),
                    2 => {
                        let color = format!("\x1b[0;{}m", 41 + y % 7);
                        format!("{}   {}", color, "\x1b[0m")
                    }
                    3 => "<=>".to_string(),
                    4 => " 😂".to_string(),
                    _ => return Err("malfunctioning intcode program".into()),
                }
            );
            print!("\x1b[1;1H");
        }

        std::io::stdout().flush()?;

        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let intcode = &mut Intcode::new(input)?;

    let ans = run_game(intcode, 0)?
        .tiles
        .iter()
        .filter(|(_, kind)| **kind == 2)
        .count();

    Ok(ans)
}

pub fn q2(input: String, args: &[String]) -> DynResult<isize> {
    let delay = match args.get(0) {
        None => 0,
        Some(v) => v.parse::<u64>()?,
    };

    let intcode = &mut Intcode::new(input)?;
    intcode.mem().write(0, 2);

    let final_score = run_game(intcode, delay)?.score;
    Ok(final_score)
}
