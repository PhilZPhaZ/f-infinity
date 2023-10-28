mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1 = SmallNumber::new("2.345");
    let n2 = SmallNumber::new("3.0");

    let n3 = n1 / n2;

    println!("{}", n3);
}
