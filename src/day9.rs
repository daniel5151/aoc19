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
    intcode::run::to_completion(&mut intcode, &mut vec![2], &mut output)?;
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

    #[test]
    fn q1_e2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let output = q1(input.to_string(), &[]).unwrap();
        assert_eq!(output, vec![1219070632396864]);
    }

    #[test]
    fn q1_e3() {
        let input = "104,1125899906842624,99";
        let output = q1(input.to_string(), &[]).unwrap();
        assert_eq!(output, vec![1125899906842624]);
    }
}
