use luhn::*;

fn main() {
    let mut digits = std::env::args().nth(1).unwrap();
    println!("{:?}", luhn_check(&digits));
    let _ = digits.pop().unwrap();
    let c = luhn_digit(&digits).unwrap();
    println!("{:?}", c);
    digits.push(c);
    println!("{}", digits);
    luhn_check(&digits).unwrap();
}
