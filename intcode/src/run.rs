//! A collection of intcode runners for various common use-cases.

use crate::{Intcode, Result};

use std::collections::VecDeque;

/// Run the intcode interpreter without any I/O, returning an error if any read
/// or write instruction is encountered.
pub fn headless(intcode: &mut Intcode) -> Result<()> {
    while intcode.step(
        || Err("intcode cannot read input in headless mode".into()),
        |_| Err("intcode cannot write output in headless mode".into()),
    )? {}
    Ok(())
}

/// Run the intcode interpreter using stdin for input, and stdout for output.
pub fn interactively(intcode: &mut Intcode) -> Result<()> {
    while intcode.step(
        || {
            print!("> ");
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            Ok(buf.trim().parse::<isize>()?)
        },
        |i| {
            println!("{}", i);
            Ok(())
        },
    )? {}
    Ok(())
}

/// Run the intcode interpreter to completion using the provided input and
/// output buffers. Returns an error if the input Vec is exhausted.
pub fn to_completion(
    intcode: &mut Intcode,
    input: &mut Vec<isize>,
    output: &mut Vec<isize>,
) -> Result<()> {
    input.reverse();

    while intcode.step(
        || {
            input
                .pop()
                .ok_or_else(|| "no more input in the input buffer".into())
        },
        |i| {
            output.push(i);
            Ok(())
        },
    )? {}
    Ok(())
}

/// Run the intcode interpreter with the provided input until the machine
/// has outputted `n` values. If the machine halts, None is returned.
/// Returns an error if the input VecDeque is exhausted.
pub fn until_output(intcode: &mut Intcode, input: &mut VecDeque<isize>) -> Result<Option<isize>> {
    let mut output = None;
    loop {
        let running = intcode.step(
            || {
                input
                    .pop_front()
                    .ok_or_else(|| "no more input in the input buffer".into())
            },
            |i| {
                output = Some(i);
                Ok(())
            },
        )?;

        if !running {
            return Ok(None);
        }

        if let Some(output) = output {
            return Ok(Some(output));
        }
    }
}
