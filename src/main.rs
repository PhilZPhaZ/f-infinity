mod smallnumber;
mod bignumber;

use smallnumber::smallnumber::SmallNumber;
use bignumber::bignumber::BigNumber;

fn main() {
    let n1 = BigNumber::new("0");
    let n2 = BigNumber::new("987654321");

    let n3 = n1 * n2;

    println!("{}", n3);
}
