mod smallnumber;
mod bignumber;

use smallnumber::smallnumber::SmallNumber;

fn main() {
    let n1: SmallNumber = SmallNumber::new("-2.3");
    let n2: SmallNumber = SmallNumber::new("-15.5");

    let n3: SmallNumber = n1 - n2;

    println!("{}", n3);
}
