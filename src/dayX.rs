use crate::DynResult;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        // do things
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let input = munge_input!(input);
    Ok(())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let input = munge_input!(input);
    Ok(())
}
