use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        // do things
        input.split('\n')
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let input = munge_input!(input);

    let _ = input;

    Ok(())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let input = munge_input!(input);

    let _ = input;

    Ok(())
}
