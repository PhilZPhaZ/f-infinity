use crate::bignumber::bignumber::BigNumber;
use super::super::SmallNumber;
use std::fmt;

pub struct RationalNumber {
    pub numerator: SmallNumber,
    pub denominator: SmallNumber,
    pub signe: bool,
    pub precision: BigNumber,
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

impl RationalNumber {
    #[warn(dead_code)]
    pub fn new(numerator: &str, denominator: &str) -> RationalNumber {
        let numerator: SmallNumber = SmallNumber::new(numerator);
        let denominator: SmallNumber = SmallNumber::new(denominator);

        let signe: bool = numerator.signe == denominator.signe;

        let precision = BigNumber::new("100");

        let mut result = RationalNumber {
            numerator,
            denominator,
            signe,
            precision
        };
        result.remove_useless_zeros();
        result
    }

    #[warn(dead_code)]
    pub fn change_precision(&mut self, precision: &str) {
        self.precision = BigNumber::new(precision);
    }

    pub fn remove_useless_zeros(&mut self) {
        self.numerator.remove_zeros_decimal();
        self.denominator.remove_zeros_decimal();
    }
}

// impl to small_number
impl RationalNumber {
    pub fn to_small_number(&mut self) -> SmallNumber {
        // get the decimal representation of the number
        let mut decimal = self.numerator.clone();

        // divide the numerator by the denominator

        SmallNumber::new("2.0")
    }
}