mod smallnumber;
mod bignumber;
mod rationalnumber;

use smallnumber::smallnumber::SmallNumber;
use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;

fn main() {
    let n1 = RationalNumber::new("2.0", "3.0");
    let n2 = RationalNumber::new("3.0", "5.0");

    let n3 = n1 + n2;

    println!("{}", n3);
}
