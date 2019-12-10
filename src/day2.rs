use crate::prelude::*;

pub fn q1(input: String, _args: &[String]) -> DynResult<isize> {
    let mut intcode = Intcode::new(input)?;
    intcode.mem().write(1, 12);
    intcode.mem().write(2, 2);
    intcode::run::headless(&mut intcode)?;
    Ok(intcode.mem().read(0))
}

pub fn q2(input: String, _args: &[String]) -> DynResult<isize> {
    let mut intcode = Intcode::new(input)?;
    let len = intcode.mem().base_len() as isize;
    for noun in 0..len {
        for verb in 0..len {
            intcode.reset();
            intcode.mem().write(1, noun);
            intcode.mem().write(2, verb);
            intcode::run::headless(&mut intcode)?;
            let res = intcode.mem().read(0);
            if res == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err("Could not find a valid (noun, verb)".into())
}
