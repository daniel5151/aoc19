# Intcode

An extensible intcode interpreter.

The `Intcode::step` function uses callbacks for input/output, making it easy to
implement custom "frontends" for the machine. Several common front-ends are
bundled with the interpreter.

## Usage

```rust
use intcode::{Intcode, Result as IntcodeResult};

fn dayX_questionX(input: String) -> IntcodeResult<()> {
    // Create a new intcode machine from a intcode program String
    let mut intcode = Intcode::new(input)?;

    // There are different built-in frontends to choose from:

    // 0. Interactive
    //   - Get input from stdin, write output to stdout
    intcode.run_interactive()?;

    // 1. Headless (day2)
    //   - Consumes any outputs
    //   - Throws an error if input is requested
    intcode.mem().write(1, noun);
    intcode.mem().write(2, verb);
    intcode.run_headless()?;
    let res = intcode.mem().read(0);

    // 2. Run To Completion (day5)
    //   - Appends any outputs to the provided output buffer
    //   - Throws an error if input buffer is exhausted
    let mut output = Vec::new();
    intcode.run_to_completion(&mut vec![1], &mut output)?;

    // 3. Run Until Output (day7)
    //   - Returns output as soon as it becomes available (pausing the machine)
    //   - Throws an error if input buffer is exhausted
    let mut input = std::collections::VecDeque::new();
    while let Some(out) = intcode.run_until_output(&mut input)? {
        input.push_back(1);
    }

    // Custom front-ends are implemented by calling intcode.step() directly,
    // providing the input/output callbacks yourself.

    // e.g: A front-end which inputs an infinite stream of `1`s, and keeps track
    // of the last output value
    let mut last_out = None;
    while intcode.step(
        || Ok(1),
        |o| {
            last_out = Some(o);
            Ok(())
        }
    ) {}

    // **Note** Instead of allocating a new intcode machine on each run, reset
    // the machine instead!
    let mut max = 0;
    for param in 0..10 {
        intcode.run_headless()?;
        max = max.max(intcode.mem().read(0));
        intcode.reset();
    }

    Ok(())
}
