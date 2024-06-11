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

        RationalNumber {
            numerator,
            denominator,
            signe,
            precision
        }
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
        let mut result: String = String::new();

        if !self.signe {
            result.push_str("-");
        }

        let mut numerator: SmallNumber = self.numerator.clone();
        let mut denominator: SmallNumber = self.denominator.clone();
        numerator.remove_zeros_decimal();
        denominator.remove_zeros_decimal();

        let numerator_len_decimal: BigNumber = numerator.len_decimal();
        let denominator_len_decimal: BigNumber = denominator.len_decimal();
        let max_len: BigNumber = {
            if numerator_len_decimal > denominator_len_decimal {
                numerator_len_decimal
            } else {
                denominator_len_decimal
            }
        };

        for _ in max_len {
            denominator = denominator * SmallNumber::new("10.0");
            numerator = numerator * SmallNumber::new("10.0");
        };

        let precision_clone: BigNumber = self.precision.clone();
        for _ in precision_clone {
            
        }

        numerator.remove_zeros_decimal();
        denominator.remove_zeros_decimal();

        println!("{} et {}", numerator, denominator);

        SmallNumber::new("2.0")
    }
}