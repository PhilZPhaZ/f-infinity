use super::super::super::bignumber::bignumber::BigNumber;

use super::super::super::smallnumber::smallnumber::SmallNumber;
use std::ops::Div;

impl Div for BigNumber {
    type Output = SmallNumber;

    fn div(self, rhs: Self) -> Self::Output {
        unimplemented!( "division not implemented for big numbers" )
    }
}
