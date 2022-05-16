/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut mcode: String = code.to_string();
    mcode.retain(|c| !c.is_whitespace());

    if mcode.len() <= 1 || mcode.chars().any(|c| !c.is_digit(10)) {
        return false;
    }

    let sum_double_digits = mcode
        .chars()
        .rev()
        .skip(1)
        .step_by(2)
        .map(|c| {
            let digit = c.to_digit(10).unwrap();
            let mut new_num = digit * 2;
            if new_num > 9 {
                new_num -= 9;
            }
            new_num
        })
        .sum::<u32>();

    let sum_rest = mcode
        .chars()
        .rev()
        .step_by(2)
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    (sum_double_digits + sum_rest) % 10 == 0
}
