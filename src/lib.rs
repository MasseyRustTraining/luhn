// From https://google.github.io/comprehensive-rust/testing/exercise.html

/// Return [Some] Luhn digit count and raw checksum for the
/// input, or [None] on syntax error.
fn luhn_sum(cc_number: &str) -> Option<(usize, u32)> {
    let mut sum = 0;
    let mut count = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            count += 1;
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else if c != ' ' {
            return None;
        }
    }

    Some((count, sum))
}

/// Basic Luhn check. Returns `true` for valid
/// input and `false` for both syntax error and
/// checksum failure.
pub fn luhn_check(cc_number: &str) -> bool {
    if let Some((count, sum)) = luhn_sum(cc_number) {
        count >= 2 && sum % 10 == 0
    } else {
        false
    }
}

/// Produce [Some] character representing a digit that
/// can be appended to `cc_number` to make it pass the
/// [luhn_check()]. Produce [None] if `cc_number` is not
/// syntactically valid.
pub fn luhn_digit(cc_number: &str) -> Option<char> {
    let mut cc_number = cc_number.to_string();
    cc_number += "0";
    if let Some((count, sum)) = luhn_sum(&cc_number) {
        if count >= 2 {
            let r = sum % 10;
            let d = (10 - r) % 10;
            Some(char::from_digit(d, 10).unwrap())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn_check("4263 9826 4026 9299"));
        assert!(luhn_check("4539 3195 0343 6467"));
        assert!(luhn_check("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn_check("4223 9826 4026 9299"));
        assert!(!luhn_check("4539 3195 0343 6476"));
        assert!(!luhn_check("8273 1232 7352 0569"));
        assert!(!luhn_check(""));
        assert!(!luhn_check("0"));
        assert!(!luhn_check("1"));
        assert!(!luhn_check("7992-7398-713"));
    }
}
