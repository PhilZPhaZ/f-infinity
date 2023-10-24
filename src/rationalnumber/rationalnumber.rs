use super::super::SmallNumber;

pub struct RationalNumber {
    numerator: SmallNumber,
    denominator: SmallNumber,
    signe: bool
}

impl RationalNumber {
    pub fn new(numerator: &str, denominator: &str) -> RationalNumber{
        let numerator: SmallNumber = SmallNumber::new(numerator);
        let denominator: SmallNumber = SmallNumber::new(denominator);

        let signe: bool = numerator.signe == denominator.signe;

        RationalNumber {
            numerator,
            denominator,
            signe
        }
    }
}