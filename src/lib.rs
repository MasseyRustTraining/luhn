// From https://google.github.io/comprehensive-rust/testing/exercise.html

use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LuhnError {
    #[error("invalid luhn syntax: {0:?}")]
    SyntaxError(String),
    #[error("luhn check failed")]
    CheckFailed,
}

/// Return [Some] Luhn digit count and raw checksum for the
/// input, or [None] on syntax error.
fn luhn_sum(cc_number: &str, mut odd: bool) -> Result<(usize, u32), LuhnError> {
    let mut sum = 0;
    let mut count = 0;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            count += 1;
            sum += if odd {
                let double_digit = digit * 2;
                double_digit % 10 + double_digit / 10
            } else {
                digit
            };
            odd = !odd;
        } else if c != ' ' {
            return Err(LuhnError::SyntaxError(cc_number.to_string()));
        }
    }

    Ok((count, sum))
}

/// Basic Luhn check. Returns `true` for valid
/// input and `false` for both syntax error and
/// checksum failure.
///
/// # Deprecation
///
/// [luhn_check()] provides more information about failures
/// and should be used instead.
#[deprecated]
pub fn luhn(cc_number: &str) -> bool {
    luhn_check(cc_number).is_ok()
}

/// Luhn check. Returns `()` for valid input,
/// and [LuhnError] for validation failure or
/// invalid syntax.
pub fn luhn_check(cc_number: &str) -> Result<(), LuhnError> {
    let (count, sum) = luhn_sum(cc_number, false)?;
    if count < 2 || sum % 10 != 0 {
        return Err(LuhnError::CheckFailed);
    }
    Ok(())
}

/// Produce a character representing a digit that
/// can be appended to `cc_number` to make it pass the
/// [luhn_check()]. Returns an error if `cc_number` is not
/// syntactically valid.
pub fn luhn_digit(cc_number: &str) -> Result<char, LuhnError> {
    let (count, sum) = luhn_sum(cc_number, true)?;
    if count < 2 {
        return Err(LuhnError::SyntaxError(cc_number.to_string()));
    }

    let r = sum % 10;
    let d = (10 - r) % 10;
    Ok(char::from_digit(d, 10).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use LuhnError::*;

    fn test_all(s0: &str, expect: Result<(), LuhnError>) {
        if s0.is_empty() {
            assert_eq!(Err(SyntaxError("".to_string())), expect);
            return;
        }
        let mut s = s0.to_string();
        let d = s.pop();
        if s.is_empty() {
            s.push(d.unwrap());
            assert_eq!(Err(SyntaxError(s)), expect);
            return;
        }
        #[allow(deprecated)]
        let l = luhn(s0);
        match expect {
            Ok(()) => {
                assert!(luhn_check(s0).is_ok());
                let d = d.unwrap();
                assert!(luhn_digit(&s).unwrap() == d);
                assert!(l);
            }
            Err(e) => {
                assert_eq!(Err(e.clone()), luhn_check(s0), "{}", s0);
                match e {
                    SyntaxError(mut s) => {
                        let _ = s.pop();
                        let d = luhn_digit(&s);
                        s.push('0');
                        assert_eq!(Err(SyntaxError(s)), d);
                    }
                    CheckFailed => {
                        let d = luhn_digit(&s).unwrap();
                        assert!(d != s0.chars().last().unwrap());
                    }
                }
                assert!(!l);
            }
        }
    }

    #[test]
    fn test_valid_cc_number() {
        // should end in 8
        test_all("4263 9826 4026 9299", Ok(()));
        test_all("4539 3195 0343 6467", Ok(()));
        test_all("7992 7398 713", Ok(()));
    }

    fn test_syntax_error(s: &str) {
        test_all(s, Err(SyntaxError(s.to_string())));
    }

    #[test]
    fn test_invalid_cc_number() {
        // should succeed
        test_all("4223 9826 4026 9299", Err(CheckFailed));
        test_all("4539 3195 0343 6465", Err(CheckFailed));
        test_all("8273 1232 7352 0563", Err(CheckFailed));
        test_syntax_error("");
        test_syntax_error("0");
        test_syntax_error("1");
        test_syntax_error("7992-7398-713");
    }
}
