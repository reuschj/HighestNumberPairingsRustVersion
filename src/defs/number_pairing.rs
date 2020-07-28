use std::fmt::Display;
use std::fmt::Formatter;
use std::result::Result;
use std::fmt::Error;
use std::cmp::{ Eq, PartialEq };
use std::cmp::{ Ord, PartialOrd, Ordering };
use std::hash::{ Hash, Hasher };
use std::marker::Copy;
use std::clone::Clone;
use crate::util::format_float;

pub struct NumberPairing {
    one_number: f64,
    pub sum: f64,
}

impl NumberPairing {
    pub fn first(&self) -> f64 { self.one_number }
    pub fn set_first(&mut self, requested_number: f64) {
        self.one_number = self.validate_and_correct(requested_number);
    }

    pub fn second(&self) -> f64 { self.sum - self.one_number }
    pub fn set_second(&mut self, requested_number: f64) {
        self.one_number = self.sum - self.validate_and_correct(requested_number);
    }

    pub fn product(&self) -> f64 { self.one_number * self.second() }
    pub fn difference(&self) -> f64 { (self.one_number - self.second()).abs() }
    pub fn result(&self) -> f64 { self.product() * self.difference() }

    // Initializers ---------------------------------------------------------- /

    pub fn new(requested_number: f64, sum: f64) -> Self {
        Self { one_number: Self::validate_and_correct_input(requested_number, &sum), sum }
    }

    pub fn default(requested_number: f64) -> Self {
        let sum = Self::default_sum();
        Self { one_number: Self::validate_and_correct_input(requested_number, &sum), sum }
    }

    // Methods --------------------------------------------------------------- /

    /// Finds the difference between two NumberPairings
    pub fn difference_from(&self, other: &Self) -> f64 { (self.result() - other.result()).abs() }

    /// This will test if two results are close enough to be considered equal to each other
    /// The two NumberPairings may still be !=
    pub fn is_equivalent_to(&self, other: &Self) -> bool { self.difference_from(other) < Self::minimum_precision() }

    // Private Methods ------------------------------------------------------- /

    /// This will set a bound to ensures that the number is positive and not more than the sum
    fn validate_and_correct(&self, requested_number: f64) -> f64 {
        let Self { sum, .. } = self;
        Self::validate_and_correct_input(requested_number, &sum)
    }

    // Static ------------------------------------------------------- /

    /// This will set a bound to ensures that the number is positive and not more than the sum
    fn validate_and_correct_input(requested_number: f64, sum: &f64) -> f64 {
        let non_negative = requested_number.abs();
        if non_negative > *sum { *sum } else { non_negative }
    }

    /// The default version of this problem is two numbers that add to 8
    pub fn default_sum() -> f64 { 8.0 }
    
    /// The minimum level of precision we care about... beyond this point, we'll consider values equal
    fn minimum_precision() -> f64 { 0.0000000001 }
}

impl Display for NumberPairing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let Self { one_number: first, sum } = self;
        let precision = 4;
        let first_rnd = format_float(first, &precision);
        let second = format_float(&self.second(), &precision);
        let difference = format_float(&self.difference(), &precision);
        let product = format_float(&self.product(), &precision);
        let result = format_float(&self.result(), &precision);
        write!(f, "{} and {} -> {} (difference: {}, product: {} -> result: {})", first_rnd, second, sum, difference, product, result)
    }
}

impl PartialEq for NumberPairing {
    fn eq(&self, other: &Self) -> bool {
        let sums_are_equal = self.sum == other.sum;
        let stored_are_equal = self.first() == other.first();
        let stored_is_equal_to_inverse = self.first() == other.second();
        (sums_are_equal && stored_are_equal) || (sums_are_equal && stored_is_equal_to_inverse)
    }
}

impl Eq for NumberPairing {}

impl PartialOrd for NumberPairing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NumberPairing  {
    fn cmp(&self, other: &Self) -> Ordering {
        let l_result = self.result();
        let r_result = other.result();
        if l_result > r_result {
            Ordering::Greater
        } else if l_result < r_result {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Hash for NumberPairing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Self{ one_number, sum } = self;
        let (l1 ,l2) = util::split_float_64(one_number);
        let (r1 ,r2) = util::split_float_64(sum);
        l1.hash(state);
        l2.hash(state);
        r1.hash(state);
        r2.hash(state);
    }
}

impl Copy for NumberPairing { }

impl Clone for NumberPairing {
    fn clone(&self) -> Self {
        *self
    }
}

mod util {
    pub fn split_float_64(float_64: &f64) -> (u32, u32) {
        let pos: f64 = float_64.abs();
        let whole: u32 = pos as u32;
        let rem: u32 = ((pos - (pos.round())) * 100_000_000.0) as u32;
        (whole, rem)
    } 
}
