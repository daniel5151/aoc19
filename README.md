# Advent of Code 2019

My solutions to Advent of Code 2019.

Some goals:

- Solve the questions (duh)
- Keep the code clean (comments when applicable, using idiomatic Rust, etc...)
- Solutions should have _reasonable_ (i.e: not strictly the _best_) space and time complexity
- Solutions should run fairly quickly (on modern PCs)

Some non-goals:

- Scoring super high on the leaderboard (timezones give people an unfair advantage, and late-night-coding isn't prime-time for me)

## Running

```bash
./aoc19.sh <day> <question>
# e.g: ./aoc19 3 1
```

The harness will automatically download question input if a `cookie.txt` is provided. It's contents should look something like this:

```
ru=53616c...; session=53616c...
```

Getting this cookie is fairly straightforward:
- Open Chrome
- Navigate to _any_ day's input URL (e.g: https://adventofcode.com/2019/day/1/input)
- Open the Chrome Network Inspector
- Refresh the URL
- Right click the `input` request, and "copy > copy as cURL"
    - the string should include a `-H 'cookie: <cookie.txt>'` component.

Alternatively, you can just invoke `cargo run --release -- <day> <question>` manually, though it will not automatically download input data.
