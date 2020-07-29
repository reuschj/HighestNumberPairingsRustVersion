#![allow(dead_code)]

mod defs;
mod util;

use crate::defs::number_pairing_problem::NumberPairingProblem;
use crate::util::{ make_line, format_float };

fn main() {
    let number_pairing_problem = NumberPairingProblem::solve_with(8.0, true);
    let intro = format!("Problem:\nFind two numbers that add up to {}, such that the product multiplied by the difference produces the largest possible value.", format_float(&number_pairing_problem.sum, &0));
    println!("\n{}\n\n{}\n{}{}\n", make_line(15), intro, number_pairing_problem, make_line(15));
}
