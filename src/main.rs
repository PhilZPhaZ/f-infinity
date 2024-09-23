mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let mut n1 = BigNumber::new("2204");
    let mut n2 = BigNumber::new("43");

    let result = n1 / n2;

    println!("{}", result);
}
