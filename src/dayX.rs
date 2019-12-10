use crate::prelude::*;

// pub fn q1(input: String, _args: &[String]) -> DynResult<Vec<isize>> {
//     let mut intcode = Intcode::new(input)?;
//     let mut output = Vec::new();
//     intcode::run::to_completion(&mut intcode, &mut vec![1], &mut output)?;
//     Ok(output)
// }

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "...";
        let output = q1(input.to_string(), &[]).unwrap();
        assert_eq!(output, ());
    }
}
