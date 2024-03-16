use luhn::*;

fn main() {
    let mut digits = std::env::args().nth(1).unwrap();
    println!("{:?}", luhn_check(&digits));
    let _ = digits.pop();
    println!("{:?}", luhn_digit(&digits));
}
