mod smallnumber;
mod bignumber;

use smallnumber::smallnumber::SmallNumber;
use bignumber::bignumber::BigNumber;

fn main() {
    let n1 = SmallNumber::new("2.12345");
    let n2 = SmallNumber::new("0.0024");

    let n3 = n1 * n2;

    println!("{}", n3);
}
