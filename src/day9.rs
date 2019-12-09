use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
        input.split('\n')
    }};
}

pub fn q1(input: String, _args: &[String]) -> DynResult<Vec<isize>> {
    let mut intcode = Intcode::new(input)?;
    let mut output = Vec::new();
    intcode.run_to_completion(&mut vec![1], &mut output)?;
    Ok(output)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<Vec<isize>> {
    let mut intcode = Intcode::new(input)?;
    let mut output = Vec::new();
    intcode.run_to_completion(&mut vec![2], &mut output)?;
    Ok(output)
}

#[cfg(test)]
#[allow(clippy::let_unit_value)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let output = q1(input.to_string(), &[]).unwrap();
        assert_eq!(
            output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
}
