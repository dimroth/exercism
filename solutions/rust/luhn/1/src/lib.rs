/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let chars = &code.chars().rev().filter(|c| !c.is_whitespace());

    if chars.clone().count() <= 1 {
        return false;
    }

    let mut sum = 0;

    for (index, c) in chars.clone().enumerate() {
        if !c.is_digit(10) {
            return false;
        }

        let digit = c.to_digit(10).unwrap();
        if index % 2 == 0 {
            sum += digit;
        } else {
            sum += if digit * 2 > 9 { digit * 2 - 9 } else { digit * 2 };
        }
    }

    sum % 10 == 0
}
