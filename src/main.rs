mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1: SmallNumber =  SmallNumber::new("123.456");
    let n2: SmallNumber  = SmallNumber::new("789.123");

    let n3: SmallNumber = n1 * n2;

    println!("n3: {}", n3);
}
