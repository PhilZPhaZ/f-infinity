use super::super::super::bignumber::bignumber::BigNumber;
use super::super::smallnumber::{SmallNumber, VecU8};

impl std::ops::Sub for SmallNumber {
    type Output = SmallNumber;

    fn sub(self, rhs: Self) -> Self::Output {
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

        let self_number_abs = self_number.set_signe_to_positive();
        let rhs_number_abs = rhs_number.set_signe_to_positive();

        // find the biggest number to have the signe and to create the biggest and smallest number variable
        let (biggest_number, smallest_number) = if self_number_abs > rhs_number_abs {
            (self_number.clone(), rhs_number.clone())
        } else {
            (rhs_number.clone(), self_number.clone())
        };

        // signe
        let signe = if self.signe == rhs.signe {
            if rhs_number > self_number {
                false
            } else {
                true
            }
        } else {
            if self.signe == false {
                false
            } else {
                true
            }
        };

        // addition process
        let mut addition: BigNumber;

        if self.signe == rhs.signe {
            addition = biggest_number - smallest_number;
        } else {
            addition = biggest_number + smallest_number;
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
    }
}
