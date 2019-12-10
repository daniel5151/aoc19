# Intcode

An extensible intcode interpreter.

While the intcode machine doesn't define a single `run` function directly, the
machine's `step` function uses _callbacks_ for input/output, making it easy to
implement custom "runners" for the machine.

## Usage

```rust
use intcode::{Intcode, Result as IntcodeResult};

fn dayX_questionX(input: String) -> IntcodeResult<()> {
    // Create a new intcode machine from an intcode program String
    let intcode = &mut Intcode::new(input)?;

    // The library comes pre-packaged with several runners

    // 0. Interactive
    //   - Get input from stdin, write output to stdout
    intcode::run::interactive()?;

    // 1. Headless (day2)
    //   - Consumes any outputs
    //   - Throws an error if input is requested
    intcode.mem().write(1, noun);
    intcode.mem().write(2, verb);
    intcode::run::headless(intcode)?;
    let res = intcode.mem().read(0);

    // 2. Run To Completion (day5)
    //   - Appends any outputs to the provided output buffer
    //   - Throws an error if input buffer is exhausted
    let mut output = Vec::new();
    intcode::run::to_completion(intcode, &mut vec![1], &mut output)?;

    // 3. Run Until Output (day7)
    //   - Returns output as soon as it becomes available (pausing the machine)
    //   - Throws an error if input buffer is exhausted
    let mut input = std::collections::VecDeque::new();
    while let Some(out) = intcode::run::until_output(intcode, &mut input)? {
        input.push_back(1);
    }

    // Custom runners can be implemented by calling intcode.step() directly,
    // providing the input/output callbacks yourself.

    // e.g: Run the intcode machine with an infinite stream of `1`s as input,
    // and only keeping track of the last outputted value
    let mut last_out = None;
    while intcode.step(
        || Ok(1),
        |o| {
            last_out = Some(o);
            Ok(())
        }
    ) {}

    // **Protip** Instead of allocating a new intcode machine on each run, reset
    // the machine instead!
    let mut max = 0;
    for param in 0..10 {
        intcode::run::headless(intcode)?;
        max = max.max(intcode.mem().read(0));
        intcode.reset();
    }

    Ok(())
}
