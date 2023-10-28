mod smallnumber;
mod bignumber;
mod rationalnumber;

use smallnumber::smallnumber::SmallNumber;
use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;

fn main() {
    let n1 = SmallNumber::new("2.345");
    let n2 = SmallNumber::new("3.0");

    let n3 = n1 / n2;

    println!("{}", n3);
}
