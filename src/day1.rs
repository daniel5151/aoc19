use crate::prelude::*;

fn req_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn fuel_fuel(fuel: usize) -> usize {
    match fuel {
        0 => 0,
        f => f + fuel_fuel(req_fuel(f)),
    }
}

pub fn q1(input: String, _args: &[String]) -> DynResult<usize> {
    input.split('\n').try_fold(0, |a, ln| -> DynResult<_> {
        let mass = ln.parse::<usize>()?;
        Ok(a + req_fuel(mass))
    })
}

pub fn q2(input: String, _args: &[String]) -> DynResult<usize> {
    input.split('\n').try_fold(0, |a, ln| -> DynResult<_> {
        let mass = ln.parse::<usize>()?;
        let fuel = req_fuel(mass);

        Ok(a + fuel_fuel(fuel))
    })
}
