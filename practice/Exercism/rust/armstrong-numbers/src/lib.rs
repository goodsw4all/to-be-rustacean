fn num_to_digits(mut num: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    loop {
        let digit = num % 10;
        digits.push(digit);
        num = num / 10;
        if num == 0 {
            break;
        }
    }

    println!("{num} -> {digits:?}");
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num_to_digits(num);
    let exponent = digits.len() as u32;

    let amstrong = digits.iter().fold(0, |sum, elem| sum + elem.pow(exponent));

    amstrong == num
}
