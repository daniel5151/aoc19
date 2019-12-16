use crate::prelude::*;

macro_rules! munge_input {
    ($input:ident) => {{
        let input = &$input;

        const E: &str = "invalid input";

        let mut rxns = HashMap::new();

        for ln in input.split('\n') {
            let mut react = ln.split("=>");
            let ls = react.next().ok_or(E)?.trim().split(", ");
            let mut rs = react.next().ok_or(E)?.trim().split(' ');

            let product_n = rs.next().ok_or(E)?.trim().parse::<usize>()?;
            let product = rs.next().ok_or(E)?.trim().to_string();

            rxns.insert(product.clone(), (product_n, Vec::new()));
            let reactants = &mut rxns.get_mut(&product).unwrap().1;

            for reactant in ls {
                let mut reactant = reactant.split(' ');
                let reactant_n = reactant.next().ok_or(E)?.trim().parse::<usize>()?;
                let reactant = reactant.next().ok_or(E)?.trim().to_string();
                reactants.push((reactant_n, reactant));
            }
        }

        rxns
    }};
}

fn req_ore<'a>(
    rxns: &'a HashMap<String, (usize, Vec<(usize, String)>)>,
    excess: &mut HashMap<&'a str, usize>,
    fuel: usize,
) -> DynResult<usize> {
    let mut reqs: VecDeque<(usize, &str)> = VecDeque::new();

    let mut ore = 0;

    reqs.push_back((fuel, "FUEL"));
    while let Some((mut req_n, product)) = reqs.pop_front() {
        if product == "ORE" {
            ore += req_n;
            continue;
        }

        if let Some(excess) = excess.get_mut(&product) {
            if *excess >= req_n {
                *excess -= req_n;
                continue;
            } else {
                req_n -= *excess;
                *excess = 0;
            }
        }

        let (multiplier, reactants) = rxns.get(product).expect("broken invariant");
        let iters = (req_n as f32 / *multiplier as f32).ceil() as usize;

        *excess.entry(product).or_default() += (iters * multiplier) - req_n;

        for (reactant_n, reactant) in reactants {
            reqs.push_back((reactant_n * iters, reactant))
        }
    }

    Ok(ore)
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    let rxns = munge_input!(input);

    let mut excess = HashMap::new();

    req_ore(&rxns, &mut excess, 1)
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    let rxns = munge_input!(input);

    let mut excess = HashMap::new();

    // must be careful with the step size. too large, and it causes a "subtract with
    // overflow" error on line 63.
    let mut step_size = 2usize.pow(10);
    let mut ore = 1000000000000;
    let mut fuel = 0;
    loop {
        let req_ore = req_ore(&rxns, &mut excess, step_size)?;
        if req_ore > ore {
            if step_size == 1 {
                break;
            }

            step_size /= 2;
            continue;
        }

        fuel += step_size;
        ore -= req_ore;
    }

    Ok(fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    const E1: &str = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";

    const E2: &str = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

    const E3: &str = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";

    #[test]
    fn q1_e1() {
        let input = E1.trim();
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 31);
    }

    #[test]
    fn q1_e2() {
        let input = E2.trim();
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 165);
    }

    #[test]
    fn q1_e3() {
        let input = E3.trim();
        let output = q1(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 13312);
    }

    #[test]
    fn q2_e1() {
        let input = E3.trim();
        let output = q2(input.to_string(), &[]);
        assert_eq!(output.unwrap(), 82892753);
    }
}
