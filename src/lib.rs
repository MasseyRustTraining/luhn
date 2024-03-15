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
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
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
///
/// [luhn_check()] provides more information about failures
/// and should be used instead.
#[deprecated]
pub fn luhn(cc_number: &str) -> bool {
    let check = luhn_check(cc_number);
    check.is_some() && check.unwrap()
}

/// Luhn check. Returns `Some(true)` for valid input,
/// `Some(false)` for validation failure, and None` for
/// invalid syntax.
pub fn luhn_check(cc_number: &str) -> Option<bool> {
    if let Some((count, sum)) = luhn_sum(cc_number) {
        Some(count >= 2 && sum % 10 == 0)
    } else {
        None
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

    fn test_all(s0: &str, flip: bool) {
        let mut s = s0.to_string();
        let d = s.pop();
        if !flip {
            #[allow(deprecated)]
            let l = luhn(s0);
            assert!(l ^ flip);
            assert!(luhn_check(s0).unwrap());
            let d = d.unwrap();
            assert!(luhn_digit(&s).unwrap() == d);
        } else {
            let c = luhn_check(s0);
            assert!(c.is_none() || !c.unwrap());
            let maybe_d = luhn_digit(&s);
            assert!(d.is_none() || maybe_d.is_none() || maybe_d != d);
        }
    }

    #[test]
    fn test_valid_cc_number() {
        test_all("4263 9826 4026 9299", false);
        test_all("4539 3195 0343 6467", false);
        test_all("7992 7398 713", false);
    }

    #[test]
    fn test_invalid_cc_number() {
        test_all("4223 9826 4026 9299", true);
        test_all("4539 3195 0343 6476", true);
        test_all("8273 1232 7352 0569", true);
        test_all("", true);
        test_all("0", true);
        test_all("1", true);
        test_all("7992-7398-713", true);
    }
}
