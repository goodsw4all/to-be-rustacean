#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    use Error::*;

    match (from_base, to_base) {
        (base, _) if base < 2 => return Err(InvalidInputBase),
        (_, base) if base < 2 => return Err(InvalidOutputBase),
        (_, _) => {
            if let Some(idx) = number
                .iter()
                .position(|d| from_base <= *d)
            {
                return Err(InvalidDigit(number[idx]));
            }
        }
    }

    let mut value = number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, digit)| acc + digit * (from_base.pow(i as u32)));

    let mut output_digits: Vec<u32> = vec![];

    loop {
        output_digits.push(value % to_base);
        value /= to_base;
        if value == 0 {
            break;
        }
    }
    output_digits.reverse();

    Ok(output_digits)
}
