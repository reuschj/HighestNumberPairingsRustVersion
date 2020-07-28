#![allow(dead_code)]

mod defs;
mod util;

use crate::defs::number_pairing_problem::NumberPairingProblem;

fn main() {
    let npp = NumberPairingProblem::solve_with(8.0, true);
    println!("{}", npp);
}
