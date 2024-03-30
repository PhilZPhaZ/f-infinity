mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1 = SmallNumber::new("4.234");
    let n2 = SmallNumber::new("3.0");

    let n3 = n1 / n2;

    n3.generate_number(5);

    println!("{}", n3);
}
