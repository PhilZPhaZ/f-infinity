mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1 = SmallNumber::new("-0.003601");
    let n2 = SmallNumber::new("-0.00647");

    let n3 = n1 - n2;
    println!("{}", n3);
}
