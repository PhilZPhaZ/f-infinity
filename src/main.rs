mod smallnumber;
mod bignumber;

use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1: SmallNumber = SmallNumber::new("1.00");
    let n2: SmallNumber = SmallNumber::new("0.0000000003");

    let n3: SmallNumber = n1 - n2;

    println!("{}", n3);
}
