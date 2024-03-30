use super::super::SmallNumber;
use super::super::BigNumber;
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

    pub fn generate_number(&self, number_of_decimal: i128) {
        let mut _final_number: Vec<String> = vec![];

        let n1: Vec<String> = self.numerator.get_number_in_vec_str();
        let n2: Vec<String> = self.denominator.get_number_in_vec_str();

        let mut number_of_decimals: i128 = 0;

        let index_first_number: usize = 0;
        let index_second_number: usize = 0;

        while number_of_decimals < number_of_decimal {
            let first_number_to_try: String = n1[index_first_number].clone();
            let first_number_bigint: BigNumber = BigNumber::new(&first_number_to_try);

            let second_number_to_try: String = n2[index_second_number].clone();
            let second_number_bigint: BigNumber = BigNumber::new(&second_number_to_try);

            if first_number_bigint > second_number_bigint {
                continue;
            }

            number_of_decimals += 1;
        }
    }
}
