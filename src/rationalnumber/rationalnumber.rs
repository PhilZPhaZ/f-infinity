use super::super::SmallNumber;
use std::fmt;

pub struct RationalNumber {
    pub numerator: SmallNumber,
    pub denominator: SmallNumber,
    pub signe: bool,
}

impl fmt::Display for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut number: String = String::new();

        number.push_str(&self.numerator.to_string());
        number.push('\n');

        let longest: usize = if self.numerator.len() > self.denominator.len() {
            self.numerator.len()
        } else {
            self.denominator.len()
        };
        // Add leading zeros to make the numbers equal length
        let mut frac_string = String::new();
        for _ in 0..longest + 1 {
            frac_string.push('-');
        }

        number.push_str(&frac_string);
        number.push('\n');
        number.push_str(&self.denominator.to_string());

        write!(f, "{}", number)
    }
}

impl RationalNumber {
    pub fn new(numerator: &str, denominator: &str) -> RationalNumber {
        let numerator: SmallNumber = SmallNumber::new(numerator);
        let denominator: SmallNumber = SmallNumber::new(denominator);

        let signe: bool = numerator.signe == denominator.signe;

        RationalNumber {
            numerator,
            denominator,
            signe,
        }
    }
}
