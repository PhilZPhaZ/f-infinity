use super::super::super::bignumber::bignumber::BigNumber;
use super::super::smallnumber::{SmallNumber, VecU8};

/*
fn zeros_in_common(s1: &str, s2: &str) -> BigNumber {
    let mut i = 0;
    while i < s1.len() && i < s2.len() && s1.chars().nth(i).unwrap() == '0' && s2.chars().nth(i).unwrap() == '0' {
        i += 1;
    }
    let mut number = i.to_string();
    BigNumber::new(&number)
}
*/

impl std::ops::Add for SmallNumber {
    type Output = SmallNumber;

    fn add(self, rhs: Self) -> Self::Output {
        // if the two decimal parts of the numbers have not the same length
        // then we add zero to the rigne to the smallest number to have the same length
        let mut lhs_decimal: Vec<u8> = self.decimal.0.clone();
        let mut rhs_decimal: Vec<u8> = rhs.decimal.0.clone();

        let difference: usize = lhs_decimal.len().abs_diff(rhs_decimal.len());

        // here the 0 are added if the two decimal parts are not the same
        if lhs_decimal.len() == rhs_decimal.len() {
        } else if lhs_decimal.len() > rhs_decimal.len() {
            for _ in 0..difference {
                rhs_decimal.push(0);
            }
        } else if lhs_decimal.len() < rhs_decimal.len() {
            for _ in 0..difference {
                lhs_decimal.push(0);
            }
        }

        // get the biggest len between the two numbers
        // and find where the dot is placed
        let len_self: BigNumber = self.len();
        let len_rhs: BigNumber = rhs.len();

        let len_max: BigNumber;
        let mut dot_place: BigNumber;

        if len_self >= len_rhs {
            len_max = len_self;
            dot_place = self.integer.len();
        } else {
            len_max = len_rhs;
            dot_place = rhs.integer.len();
        }

        // convert lhs_decimal to &str and then create a variable self_decimal_number
        // and store a big number with the value of the string
        let self_decimal_number_str: String = lhs_decimal
            .iter()
            .map(|d: &u8| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        // create also a BigNumber but with the decimal part also
        let mut self_number: BigNumber =
            BigNumber::new(&format!("{}{}", self.integer, &self_decimal_number_str));

        // Same thing for rhs_decimal
        let rhs_decimal_number_str: String = rhs_decimal
            .iter()
            .map(|d: &u8| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        let mut rhs_number: BigNumber =
            BigNumber::new(&format!("{}{}", rhs.integer, &rhs_decimal_number_str));

        // find the biggest number to have the signe
        let signe: bool = if self_number.set_signe_to_positive() > rhs_number.set_signe_to_positive() {
            true
        } else {
            false
        };

        // addition process
        let mut addition: BigNumber;

        if self.signe == rhs.signe {
            addition = self_number + rhs_number;
        } else {
            addition = self_number - rhs_number;
        }
        addition.set_signe_to_positive();
        let mut len_addition: BigNumber = addition.len();

        while len_addition != len_max {
            len_addition = len_addition - BigNumber::one();
            dot_place = dot_place + BigNumber::one();
        }

        let result: (String, Vec<u8>) = addition.separate_in_two_str(dot_place);
        let result_integer: String = result.0;
        let result_decimal: Vec<u8> = result.1;

        SmallNumber {
            integer: BigNumber::new(&result_integer),
            signe: signe,
            decimal: VecU8(result_decimal)
        }
        /*
        // if pos-pos or neg-neg
        if (self.signe && rhs.signe) || (!self.signe && !rhs.signe) {
            // first we add the two integer parts
            let mut integer: BigNumber = self.integer + rhs.integer;

            // adding the two decimal part
            let decimal_addition: BigNumber = self_decimal_number + rhs_decimal_number;

            // substract to verify if < 0
            // verify the carry
            one_ten_pow_difference = one_ten_pow_difference - decimal_addition.clone();
            if !one_ten_pow_difference.signe || one_ten_pow_difference.is_zero() {
                integer = if self.signe && rhs.signe {
                    integer + BigNumber::one()
                } else {
                    integer - BigNumber::zero()
                };
                one_ten_pow_difference.delete_first_digit();
            } else {
                one_ten_pow_difference = decimal_addition;
            };

            // we create decimal and store the result here
            let decimal: Vec<u8> = one_ten_pow_difference.digits.0;

            // change signe if neg-neg
            let new_signe: bool = if self.signe && rhs.signe { true } else { false };

            SmallNumber {
                integer: integer,
                signe: new_signe,
                decimal: VecU8(decimal),
            }
        } else {
            // set the biggest number in order to do the sub
            let (biggest_num, smallest_num) = if self_number > rhs_number {
                (self.clone(), rhs.clone())
            } else {
                (rhs.clone(), self.clone())
            };

            // we must redo this step because we change the number

            // fill with 0 if missing
            let mut lhs_decimal: Vec<u8> = biggest_num.decimal.0.clone();
            let mut rhs_decimal: Vec<u8> = smallest_num.decimal.0.clone();

            let mut difference: usize = lhs_decimal.len().abs_diff(rhs_decimal.len());

            // here the 0 are added if the two decimal parts are not the same
            if lhs_decimal.len() == rhs_decimal.len() {
                difference = lhs_decimal.len();
            } else if lhs_decimal.len() > rhs_decimal.len() {
                for _ in 0..difference {
                    rhs_decimal.push(0);
                }
                difference = lhs_decimal.len();
            } else if lhs_decimal.len() < rhs_decimal.len() {
                for _ in 0..difference {
                    lhs_decimal.push(0);
                }
                difference = rhs_decimal.len()
            }

            // create a big number with 10^difference from string
            let ten_pow_difference: String = format!("1{}", "0".repeat(difference));
            let one_ten_pow_difference: BigNumber = BigNumber::new(&ten_pow_difference);

            // convert lhs_decimal to &str and then create a variable self_decimal_number
            // and store a big number with the value of the string
            let biggest_decimal_number_dtr: String = lhs_decimal
                .iter()
                .map(|d: &u8| d.to_string())
                .collect::<Vec<_>>()
                .join("");

            let biggest_decimal_number: BigNumber = BigNumber::new(&biggest_decimal_number_dtr);

            // Same thing for rhs_decimal
            let smallest_decimal_number_str: String = rhs_decimal
                .iter()
                .map(|d: &u8| d.to_string())
                .collect::<Vec<_>>()
                .join("");

            let smallest_decimal_number: BigNumber = BigNumber::new(&smallest_decimal_number_str);
            // all of the number

            // first we add the two integer parts
            let mut integer: BigNumber = biggest_num.integer - smallest_num.integer;

            // adding the two decimal part
            let mut decimal_addition: BigNumber = biggest_decimal_number - smallest_decimal_number;

            // check the carry
            if !decimal_addition.signe {
                integer = integer - BigNumber::one();
                decimal_addition = one_ten_pow_difference - decimal_addition;
                decimal_addition.delete_first_digit()
            }

            SmallNumber {
                integer: integer,
                signe: biggest_num.signe,
                decimal: VecU8(decimal_addition.digits.0),
            }
        }
        */
    }
}
