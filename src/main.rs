mod bignumber;
mod smallnumber;

use smallnumber::SmallNumber;

fn main() {
    let n1: SmallNumber = SmallNumber::new("-0.9999");
    let n2: SmallNumber = SmallNumber::new("-0.0002");

    let n3: SmallNumber = n1 + n2;

    println!("{}", n3);
}
