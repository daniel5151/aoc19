use crate::DynResult;

fn accept(mut val: usize) -> bool {
    let mut digits = Vec::new();
    for _ in 0..6 {
        digits.push(val % 10);
        val /= 10;
    }

    digits.reverse();

    let mut adj_same = false;
    let mut increasing = true;

    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            adj_same = true;
        }

        if digits[i + 1] < digits[i] {
            increasing = false;
        }
    }

    adj_same && increasing
}

fn accept2(mut val: usize) -> bool {
    let mut digits = Vec::new();
    for _ in 0..6 {
        digits.push(val % 10);
        val /= 10;
    }

    digits.reverse();

    let mut adj_same = false;
    let mut increasing = true;

    for i in 0..5 {
        if digits[i] == digits[i + 1] {
            adj_same = true;
        }

        if digits[i + 1] < digits[i] {
            increasing = false;
        }
    }

    let mut has_2_size_run = false;

    // check for odd sized runs
    let mut run = 1;
    let mut prev = digits[0];
    for d in &digits[1..] {
        if prev == *d {
            run += 1;
        } else {
            prev = *d;
            if run == 2 {
                has_2_size_run = true;
                break;
            }
            run = 1;
        }
    }
    if run == 2 {
        has_2_size_run = true;
    }

    adj_same && increasing && has_2_size_run
}

pub fn q1(input: String, _args: &[String]) -> DynResult<()> {
    let mut input = input.split('-');
    let start = input.next().unwrap().parse::<usize>()?;
    let end = input.next().unwrap().parse::<usize>()?;

    let range = start..=end;

    let ans = range.filter(|x| accept(*x)).count();

    eprintln!("{:?}", ans);

    Ok(())
}

pub fn q2(input: String, _args: &[String]) -> DynResult<()> {
    let mut input = input.split('-');
    let start = input.next().unwrap().parse::<usize>()?;
    let end = input.next().unwrap().parse::<usize>()?;

    let range = start..=end;

    let ans = range.filter(|x| accept2(*x)).count();

    eprintln!("{:?}", ans);

    Ok(())
}
