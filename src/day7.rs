use crate::prelude::*;

#[cfg(feature = "extras")]
mod q2_threaded;

pub fn q1(input: String, _args: &[String]) -> DynResult<(isize, Vec<isize>)> {
    let mut intcode = Intcode::new(input)?;
    let mut input = Vec::new();
    let mut output = Vec::new();

    let mut max_out = (std::isize::MIN, Vec::new());

    for phases in (0..5).permutations(5) {
        let mut prev_out = 0;

        for phase in phases.iter().copied() {
            input.push(phase);
            input.push(prev_out);

            intcode::run::to_completion(&mut intcode, &mut input, &mut output)?;
            prev_out = output.pop().ok_or("expected single intcode output")?;

            if !input.is_empty() {
                return Err("amp didn't consume all it's input".into());
            }

            if !output.is_empty() {
                return Err("amp returned more output than expected".into());
            }

            intcode.reset();
        }

        if max_out.0 < prev_out {
            max_out = (prev_out, phases);
        }
    }

    Ok(max_out)
}

pub fn q2(input: String, args: &[String]) -> DynResult<(isize, Vec<isize>)> {
    #[allow(clippy::single_match)]
    match args.get(0).map(|x| x.as_str()) {
        #[cfg(feature = "extras")]
        Some("threaded") => return q2_threaded::q2(input, &[]),
        Some(_) => return Err("invalid argument".into()),
        None => {}
    }

    let base_intcode = Intcode::new(input)?;
    let mut amps = (0..5)
        .map(|_| (base_intcode.clone(), VecDeque::new()))
        .collect::<Vec<_>>();

    let mut max_out = (std::isize::MIN, Vec::new());

    for phases in (5..10).permutations(5) {
        // seed the amps with their phase
        amps.iter_mut()
            .zip(phases.iter().copied())
            .for_each(|((_, input), phase)| input.push_back(phase));

        let mut out = 0;

        // Loop until all amps are halted
        let mut running = 5;
        'outer: loop {
            for (amp, input) in &mut amps {
                input.push_back(out);
                match intcode::run::until_output(amp, input)? {
                    Some(output) => out = output,
                    None => {
                        // it didn't need the last input
                        input.pop_back();
                        running -= 1
                    }
                }
            }

            if running == 0 {
                break 'outer;
            }
        }

        // calculate the new maximum
        if max_out.0 < out {
            max_out = (out, phases);
        }

        // reuse the amps for the next run
        for (amp, input) in amps.iter_mut() {
            if !input.is_empty() {
                return Err("amp didn't consume all it's input".into());
            }
            amp.reset();
        }
    }

    Ok(max_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_e1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let output = q1(input.to_string(), &[]).unwrap();
        assert!(output == (43210, vec![4, 3, 2, 1, 0]));
    }

    #[test]
    fn q1_e2() {
        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,
            23,4,23,99,0,0";
        let output = q1(input.to_string(), &[]).unwrap();
        assert!(output == (54321, vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn q1_e3() {
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,
            7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let output = q1(input.to_string(), &[]).unwrap();
        assert!(output == (65210, vec![1, 0, 4, 3, 2]));
    }

    #[test]
    fn q2_e1() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,
            28,-1,28,1005,28,6,99,0,0,5";
        let output = q2(input.to_string(), &[]).unwrap();
        assert!(output == (139629729, vec![9, 8, 7, 6, 5]));
    }

    #[test]
    fn q2_e2() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,
            1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,
            55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let output = q2(input.to_string(), &[]).unwrap();
        assert!(output == (18216, vec![9, 7, 8, 5, 6]));
    }
}
