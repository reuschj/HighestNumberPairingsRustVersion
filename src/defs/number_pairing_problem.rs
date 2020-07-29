use super::number_pairing::NumberPairing;
use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::fmt::Error;

/**
 * Stores the results of a number pairing problem
 */
pub struct Results {
    best: f64,
    best_pairing: Vec<NumberPairing>,
    other: Option<Vec<NumberPairing>>,
}

/**
  * A structure to define a problem by which takes two numbers that sum to a given amount (default to 8).
  * The problem must find the largest number combination (determined by multiplying the difference by the product of the two numbers)
  */
pub struct NumberPairingProblem {
    pub sum: f64,
    runs_to_solve: u32,
    pub results: Option<Results>,
}

impl NumberPairingProblem {

    // Initializers ---------------------------------------------------------- /

    pub fn solve_with(initial_sum: f64, collect_other_results: bool) -> Self {
        let mut problem = Self {
            sum: initial_sum,
            runs_to_solve: 0,
            results: None,
        };
        problem.solve(collect_other_results);
        problem
    }

    pub fn solve_default() -> Self {
        Self::solve_with(8.0, true)
    }

    // Methods --------------------------------------------------------------- /

    /**
      * This method is called during initialization to get the results of the problem
      * Returns a tuple with the best result, an array of best result pairings and an array of other top pairings (sorted)
      * These values will be accessed by public getter properties
      */
      fn solve(&mut self, collect_other_results: bool) {
        // This is a NumberPairing instance that will always have a result of 0
        // We will use this as the initial high NumberPairing to beat
        let initial_high_value = NumberPairing::new(0.0, self.sum);

        /// Stores global values to pass to the recursive function
        struct SolveGlobals<'a> {
            problem: &'a mut NumberPairingProblem,
            collect_other_results: bool,

            // This is a NumberPairing instance that will always have a result of 0
            // We will use this as the initial high NumberPairing to beat
            initial_high_value: NumberPairing,

            // These constants for lower and upper bounds set the boundaries for numbers in the number pairing
            // We will use these to ensure we don't get a NumberPairing with a number outside of these bounds
            lower_bounds: f64,
            upper_bounds: f64,

            // These variable will hold the current overall best result that the recursive function will compare to and set as needed
            // At the end, these values will be returned in a tuple
            overall_best_result: NumberPairing,
            best_results: Vec<NumberPairing>,
            other_results: Option<Vec<NumberPairing>>,

            // This is a failsafe. Hopefully, we end recursion before we get here, but just in case, it sets a limit on recursion
            run_count: u32,
            max_runs: u32,
        }

        let Self { sum, .. } = *self;

        // Now, we'll set values to pass down
        let mut globals = SolveGlobals {
            problem: self,
            collect_other_results,
            initial_high_value: NumberPairing::new(0.0, sum),
            lower_bounds: 0.0,
            upper_bounds: sum / 2.0,
            overall_best_result: initial_high_value.clone(),
            best_results: Vec::new(),
            other_results: if collect_other_results { Some(Vec::new()) } else { None }, 
            run_count: 0,
            max_runs: 40,
        };

        // This is a recursive function that will start with low precision, look for the max value,
        // then continue looking for higher max values (at a higher precision) around that max value.
        // When further recursion no longer finds a better value, recursion ends (as the max value has been found)
        fn get_highest_result_of_seq(low: f64, high: f64, precision: f64, globals: &mut SolveGlobals) {
            let SolveGlobals { problem, collect_other_results, initial_high_value, max_runs, .. } = globals;

            if globals.run_count >= *max_runs { return };
            globals.run_count += 1;

            // We will set three local variables that will be for each recursive run... these will be compared to the overall variables for the method
            let mut seq_best_result: NumberPairing = initial_high_value.clone();
            let mut best_results_of_seq: Vec<NumberPairing> = Vec::new();
            let mut other_results_of_seq: Option<Vec<NumberPairing>> = if *collect_other_results { Some(Vec::new()) } else { None };

            // Closure to determine if we can add to the other sequence
            let can_be_added_to_other = |pairing: &NumberPairing| -> bool {
                *pairing != *initial_high_value && precision >= 0.01 && *collect_other_results
            };

            // Set the search range and loop through each value in it
            let multiplier = 100_000_000.0;
            let conversion = (1.0 / precision) * multiplier;
            let low_bound = (low * conversion).round() as usize;
            let high_bound = (high * conversion).round() as usize;
            for i in (low_bound..=high_bound).step_by(multiplier as usize) {
                let number = i as f64 / conversion;

                // Create a new NumberPairing to evaluate
                let this_result = NumberPairing::new(number, problem.sum);
                // println!("{}", this_result);
                if this_result > seq_best_result {
                    // If the new Result is better than any other in the sequence, it's the new max
                    // We'll set it to the best in sequence and move and previous best results to the other results array
                    // Then add the new result to the best results array
                    seq_best_result = this_result;
                    for result in &best_results_of_seq {
                        if can_be_added_to_other(&result) {
                            if let Some(other) = &mut other_results_of_seq {
                                other.push(*result);
                            }
                        }
                    }
                    best_results_of_seq.clear();
                    best_results_of_seq.push(seq_best_result);
                } else if this_result == seq_best_result {
                    // If we found a NumberPairing that matches, but doesn't exceed, the existing best, we'll add it to the best results array
                    best_results_of_seq.push(this_result);

                } else if can_be_added_to_other(&this_result) {
                    // Else, we'll just add it to the other results array
                    if let Some(other) = &mut other_results_of_seq {
                        other.push(this_result);
                    }
                }
            }

            // When the best result from the sequence is lower or equal to the overall result (or close enough), we found the max and can stop
            let condition_to_end_recursion = seq_best_result <= globals.overall_best_result || seq_best_result.is_equivalent_to(&globals.overall_best_result);
            if condition_to_end_recursion {
                problem.runs_to_solve = globals.run_count;
                return;
            }

            // In this case, the sequence produced a higher result than the previous, so we'll set it to the new overall best
            // We'll also move the previous best results from the best results array to the other results array
            // and add the new best results to the best results array
            globals.overall_best_result = seq_best_result;
            for result in &globals.best_results {
                if can_be_added_to_other(&result) {
                    if let Some(other) = &mut other_results_of_seq {
                        other.push(result.clone());
                    }
                }
            }
            globals.best_results.clear();
            globals.best_results.append(&mut best_results_of_seq);
            if let Some(other) = &mut other_results_of_seq {
                if let Some(other_globals) = &mut globals.other_results {
                    other_globals.append(other);
                }
            }
            // This finds what the first number was from the best result. This the number we'll target when call the function again
            let best_number_of_seq: f64 = globals.overall_best_result.first();
            // We will run the function again with more precision...
            let new_precision: f64 = precision / ((globals.run_count * 4) as f64);
            // We'll look to half the current precision on either side of the best value
            let margin_to_search_around_best_value: f64 = precision / 2.0;
            // ... but we'll look in a smaller range. The new result will be the best number from the sequence minus the shrink amount
            let mut new_low_value = best_number_of_seq - margin_to_search_around_best_value;
            if new_low_value < low {
                // If new start is lower than lower bounds, snap it to lower bounds
                new_low_value = low;
            }
            // ... and new end is the best number in the sequence plus the shrink amount
            let mut new_high_value = best_number_of_seq + margin_to_search_around_best_value;
            if new_high_value > high {
                // If new end is higher than upper bounds, snap it to upper bounds
                new_high_value = high;
            }

            // Call recursive function again with narrower range as defined above (but higher precision)
            get_highest_result_of_seq(new_low_value, new_high_value, new_precision, globals);
        }

        let SolveGlobals { lower_bounds, upper_bounds, .. } = globals;

        get_highest_result_of_seq(lower_bounds, upper_bounds, sum / 4.0, &mut globals);

        let SolveGlobals {
            overall_best_result,
            best_results: best_pairing,
            ..
        } = globals;
        
        // Sort the other results
        let mut others_sorted: Option<Vec<NumberPairing>> = None;
        if let Some(other_results) = &mut globals.other_results {
            other_results.sort_unstable_by(|a, b| b.cmp(a));
            other_results.dedup();
            let mut sorted: Vec<NumberPairing> = Vec::new();
            sorted.append(other_results);
            others_sorted = Some(sorted);
        }

        let best = overall_best_result.result();
        let other = others_sorted;

        // Return the results
        let results = Results { best, best_pairing, other };
        self.results = Some(results);
    }
}

impl Display for NumberPairingProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let Self { sum, runs_to_solve, results: possible_results } = self;
        if let Some(results) = possible_results {
            let Results { best, best_pairing, other } = results;
            let mut best_list = String::new();
            for number_pairing in best_pairing {
                best_list.push_str(number_pairing.to_string().as_str());
                best_list.push_str("\n");
            }
            let mut other_list = String::new();
            if let Some(other_pairings) = other {
                let max_results = if other_pairings.len() > 10 { 10 } else { other_pairings.len() };
                for index in 0..max_results {
                    let number_pairing = other_pairings.get(index).unwrap();
                    other_list.push_str(number_pairing.to_string().as_str());
                    other_list.push_str("\n");
                }
            }
            let runs_str = if *runs_to_solve == 1 { "run" } else { "runs" };
            let other_results_str = if let Some(_other_pairings) = other { format!("Other Top Results:\n{}", other_list) } else { format!("") };
            write!(f, "\nBest Result: {} (Solved in {} {})\n\nBest Number Combination:\n{}\n{}\n", best, runs_to_solve, runs_str, best_list, other_results_str)
        } else {
            write!(f, "This problem (finding a number pairing summing to {}) has not yet been solved.", sum)
        }
    }
}