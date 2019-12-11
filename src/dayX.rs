use crate::prelude::*;

// pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
//     let intcode = &mut Intcode::new(input)?;
//     let input = &mut vec![];
//     let output = &mut vec![];
//     intcode::run::to_completion(intcode, &mut input, output)?;
//     Ok(())
// }

// pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
//     let intcode =  &mut Intcode::new(input)?;
//     let input = &mut VecDeque::new();
//     input.push_back(0);
//     while let Some(output) = intcode::run::until_output(intcode, input)? {
//         eprintln!("{:?}", output);
//         // ...
//     }
//     Ok(())
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
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), ());
    }
}
