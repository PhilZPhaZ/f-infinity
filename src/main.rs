mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1 = BigNumber::new("123456789");
    let n2 = BigNumber::new("987654321");

    let n3 = n1 - n2;

    println!("{}", n3);
}
