use super::super::super::bignumber::bignumber::{BigNumber, VecU8};

use super::super::super::smallnumber::smallnumber::{SmallNumber, VecU8_SmallNumber};
use std::ops::Div;

impl Div for BigNumber {
    type Output = SmallNumber;

    fn div(self, rhs: Self) -> Self::Output {
        // rhs copy
        let rhs_copy = rhs.clone();
        let self_copy = self.clone();

        // full division
        let mut quotient: Vec<u8> = Vec::new();
        let mut remainder: Vec<u8> = Vec::new();
        let mut dividend: Vec<u8> = self.digits.0;

        // check which number has more precision
        let precision: u128 = {
            if self.division_precision > rhs.division_precision {
                self.division_precision
            } else {
                rhs.division_precision
            }
        };

        // add the len of the divisor - 1 to the precision
        let mut rhs_digits = rhs.digits.0.clone();
        let mut rhs_len: u128 = 0;

        while !rhs_digits.is_empty() {
            rhs_digits.remove(0);
            rhs_len += 1;
        }

        let precision = precision + rhs_len - 1;

        // multiply the dividend and divisor by 10^precision
        for _ in 0..precision {
            dividend.push(0);
            remainder.push(0);
        };

        // division process
        while quotient.len() != precision as usize {
            if !dividend.is_empty() {
                remainder.push(dividend.remove(0));
            } else {
                remainder.push(0);
            }

            // find the quotient digit
            let mut quotient_digit: u8 = 0;

            while BigNumber::new(&remainder.iter().map(|x| x.to_string()).collect::<String>()) >= rhs_copy {
                let temp = rhs_copy.clone();
                let mut temp_digit: u8 = 1;

                while temp.clone() * BigNumber::new(&temp_digit.to_string()) <= BigNumber::new(&remainder.iter().map(|x| x.to_string()).collect::<String>()) {
                    temp_digit += 1;
                }

                temp_digit -= 1;
                quotient_digit += temp_digit;
                remainder = (BigNumber::new(&remainder.iter().map(|x| x.to_string()).collect::<String>()) - (temp * BigNumber::new(&temp_digit.to_string()))).digits.0;
            }

            // add the quotient digit to the quotient
            quotient.push(quotient_digit);
        }

        // to determine the place of the decimal point
        // juste take the first digit of the quotient and multiply it by the divisor
        // and if the result is bigger than the dividend, it means that the decimal point is at the right place
        let mut quotient_string: String = quotient.iter().map(|x| x.to_string()).collect::<String>();

        // get the first digit of the quotient_string and create a BigNumber
        let mut temp: BigNumber = BigNumber::new(&quotient_string.remove(0).to_string());

        // index
        let mut index: BigNumber = BigNumber::zero();

        while temp.clone() * rhs_copy.clone() <= self_copy {
            temp = temp * BigNumber::new("10") + BigNumber::new(&quotient_string.remove(0).to_string());
            index = index + BigNumber::new("1");
        }

        // transform the quotient into VecU8
        let quotient_vec_u8: VecU8 = VecU8{0: quotient};

        // split the quotien_vec_u8 in two parts
        let (first_part, second_part) = quotient_vec_u8.split_in_two(index);

        // remove useless zeros in first_part
        let integer_part: BigNumber = BigNumber::new(&first_part.to_string());

        // transform the second_part into VecU8_SmallNumber
        let second_part: VecU8_SmallNumber = VecU8_SmallNumber(second_part.0);

        // handle the sign
        let signe: bool = self.signe == rhs.signe;

        SmallNumber {
            signe: signe,
            integer: integer_part,
            decimal: second_part,
        }
    }
}
