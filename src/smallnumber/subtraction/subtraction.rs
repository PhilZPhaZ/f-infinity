use super::super::super::bignumber::bignumber::BigNumber;
use super::super::smallnumber::{SmallNumber, VecU8_SmallNumber};

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

        // find the biggest len of the two decimals parts and the biggest part will be in dot_place
        let self_decimal_len = self.len_decimal();
        let rhs_decimal_len = rhs.len_decimal();
        let dot_place: BigNumber;

        if self_decimal_len > rhs_decimal_len {
            dot_place = self_decimal_len;
        } else {
            dot_place = rhs_decimal_len;
        }

        // convert lhs_decimal to &str and then create a variable self_decimal_number
        // and store a big number with the value of the string
        let self_decimal_number_str: String = lhs_decimal
            .iter()
            .map(|d: &u8| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        // create also a BigNumber but with the decimal part also
        let self_number: BigNumber =
            BigNumber::new(&format!("{}{}", self.integer, &self_decimal_number_str));

        // Same thing for rhs_decimal
        let rhs_decimal_number_str: String = rhs_decimal
            .iter()
            .map(|d: &u8| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        let rhs_number: BigNumber =
            BigNumber::new(&format!("{}{}", rhs.integer, &rhs_decimal_number_str));
        
        // copy the numbers
        let mut self_number_clone = self_number.clone();
        let mut rhs_number_clone = rhs_number.clone();
        self_number_clone.set_signe_to_positive();
        rhs_number_clone.set_signe_to_positive();
        
        // addition process
        let mut substraction: BigNumber;

        if (self.signe == rhs.signe) == true {
            substraction = self_number - rhs_number;
        } else if (self.signe == rhs.signe) == false {
            substraction = self_number + rhs_number;
        } else if self.signe == true && rhs.signe == false {
            substraction = self_number + rhs_number;
        } else {
            substraction = self_number - rhs_number;
        }

        substraction.set_signe_to_positive();
        let len_substraction: BigNumber = substraction.len();

        if len_substraction < dot_place {
            let difference: BigNumber = dot_place.clone() - len_substraction.clone();
            let mut i = BigNumber::zero();
            while i <= difference {
                i = i + BigNumber::one();
                substraction.digits.0.insert(0, 0);
            }
        }

        // find the signe
        let signe: bool;
        if self_number_clone > rhs_number_clone {
            signe = self.signe;
        } else {
            signe = !rhs.signe;
        }

        // here find the dot place
        let new_len_substraction = substraction.len();

        let dot_difference = new_len_substraction - dot_place;

        // transform the result to be useable
        let result: (String, Vec<u8>) = substraction.separate_in_two_str(dot_difference);
        let result_integer: String = result.0;
        let result_decimal: Vec<u8> = result.1;

        SmallNumber {
            signe: signe,
            integer: BigNumber::new(&result_integer),
            decimal: VecU8_SmallNumber(result_decimal),
        }
    }
}
