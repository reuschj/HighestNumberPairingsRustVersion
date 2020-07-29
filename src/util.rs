/// Makes a line
pub fn make_line(length: usize) -> String {
    "-".repeat(length)
}

/// Formats a floating point number as string
pub fn format_float(float: &f64, precision: &usize) -> String {
    let rounded = if float % 1.0 == 0.0 { format!("{:.0}", float) } else { format!("{1:.0$}", precision, float) };
    String::from(rounded.trim_matches(|a| a == '0'))
}