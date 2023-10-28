use super::super::super::SmallNumber;
use super::super::rationalnumber::RationalNumber;

impl std::ops::Mul for RationalNumber {
    type Output = RationalNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_numerator: SmallNumber = self.numerator * rhs.numerator;
        let new_denominator: SmallNumber = self.denominator * rhs.denominator;

        let new_signe: bool = new_numerator.signe == new_denominator.signe;

        RationalNumber {
            numerator: new_numerator,
            denominator: new_denominator,
            signe: new_signe,
        }
    }
}
