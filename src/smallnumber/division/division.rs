use crate::bignumber::bignumber::BigNumber;

use super::super::super::RationalNumber;
use super::super::smallnumber::SmallNumber;

impl std::ops::Div for SmallNumber {
    type Output = RationalNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let precision = BigNumber::new("100");

        let mul_number: RationalNumber = RationalNumber {
            numerator: self.clone(),
            denominator: SmallNumber::new("1.0"),
            signe: self.signe,
            precision: precision.clone(),
        };
        let div_number: RationalNumber = RationalNumber {
            numerator: SmallNumber::new("1.0"),
            denominator: rhs.clone(),
            signe: rhs.signe,
            precision: precision,
        };

        mul_number * div_number
    }
}
