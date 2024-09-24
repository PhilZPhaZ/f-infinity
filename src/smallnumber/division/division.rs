use crate::bignumber::bignumber::BigNumber;
use crate::smallnumber;

use super::super::super::RationalNumber;
use super::super::smallnumber::{SmallNumber, VecU8_SmallNumber};

impl std::ops::Div for SmallNumber {
    type Output = SmallNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let self_sn:SmallNumber = self.clone();
        let rhs_sn:SmallNumber = rhs.clone();

        // the goal is to transform the small number into a bignumber to perform the division
        // first transform the integer part into a String
        let self_integer_str: String = format!("{}", self_sn.integer);

        // then transform the decimal part into a String
        let mut self_decimal_str: String = String::new();
        for digit in self_sn.decimal.0.iter() {
            self_decimal_str.push_str(&digit.to_string());
        };

        // concatenate the integer and decimal part
        let self_str: String = format!("{}{}", self_integer_str, self_decimal_str);

        // transform the string into a bignumber
        let self_bn: BigNumber = BigNumber::new(&self_str);

        // same for the rhs
        let rhs_integer_str: String = format!("{}", rhs_sn.integer);
        let mut rhs_decimal_str: String = String::new();
        for digit in rhs_sn.decimal.0.iter() {
            rhs_decimal_str.push_str(&digit.to_string());
        };
        let rhs_str: String = format!("{}{}", rhs_integer_str, rhs_decimal_str);
        let rhs_bn: BigNumber = BigNumber::new(&rhs_str);

        // perform the division
        let result: SmallNumber = self_bn / rhs_bn;

        // determine the number of decimal place
        let self_decimal_place: BigNumber   = self_sn.decimal.len();
        let rhs_decimal_place: BigNumber  = rhs_sn.decimal.len();
        let decimal_place: BigNumber = self_decimal_place - rhs_decimal_place;

        // determine the number of decimal place
        let mut result_decimal: Vec<u8> = Vec::new();
        for digit in result.decimal.0.iter() {
            result_decimal.push(*digit);
        };

        // to do

        result
    }
}
