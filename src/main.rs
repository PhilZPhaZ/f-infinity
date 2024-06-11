mod bignumber;
mod rationalnumber;
mod smallnumber;

use bignumber::bignumber::BigNumber;
use rationalnumber::rationalnumber::RationalNumber;
use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1 = SmallNumber::new("5.6");
    let n2 = SmallNumber::new("12.34");

    let mut n3 = n1 / n2;
    n3.change_precision("14");
    
    println!("{}", n3.to_small_number());
}
