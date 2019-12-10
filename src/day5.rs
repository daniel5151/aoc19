use crate::prelude::*;

pub fn q1(input: String, _args: &[String]) -> DynResult<Vec<isize>> {
    let mut intcode = Intcode::new(input)?;
    let mut output = Vec::new();
    intcode::run::to_completion(&mut intcode, &mut vec![1], &mut output)?;
    Ok(output)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<Vec<isize>> {
    let mut intcode = Intcode::new(input)?;
    let mut output = Vec::new();
    intcode::run::to_completion(&mut intcode, &mut vec![5], &mut output)?;
    Ok(output)
}
