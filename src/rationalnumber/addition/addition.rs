use super::super::rationalnumber::RationalNumber;
use super::super::super::SmallNumber;

impl std::ops::Add for RationalNumber {
    type Output = RationalNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mul_self: SmallNumber = self.denominator.clone();
        let mul_rhs: SmallNumber = rhs.denominator.clone();

        // determinate the numerator
        let new_rhs_numerator: SmallNumber = rhs.numerator * mul_self.clone();
        let new_self_numerator: SmallNumber = self.numerator * mul_rhs.clone();

        // add the 2 number
        let new_numerator: SmallNumber = new_self_numerator + new_rhs_numerator;

        // determinate the denominator
        let new_denominator: SmallNumber = mul_self * mul_rhs;

        // determinate the signe
        let new_signe: bool = new_numerator.signe == new_denominator.signe;

        RationalNumber {
            numerator: new_numerator,
            denominator: new_denominator,
            signe: new_signe
        }
    }
}