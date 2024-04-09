use super::super::super::BigNumber;
use super::super::smallnumber::{SmallNumber, VecU8};

impl std::ops::Mul for SmallNumber {
    type Output = SmallNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_dot_placement: i128 = self.decimal.0.len().try_into().unwrap();
        let rhs_dot_placement: i128 = rhs.decimal.0.len().try_into().unwrap();

        let dot_placement: i128 = self_dot_placement + rhs_dot_placement;

        // convert BigNumber to i128
        let mut temp = self.integer.digits.0;
        let mut bytes = [0u8; 16];
        temp.copy_from_slice(&mut bytes);
        let self_integer = i128::from_le_bytes(bytes);

        // create a Vec i128 with self.integer (i128) an self.decimal.0 (Vec u8)
        let mut self_number: Vec<i128> = vec![self_integer];
        for value in self.decimal.0.iter() {
            self_number.push(*value as i128);
        }

        // same for rhs
        let mut temp = rhs.integer.digits.0;
        let mut bytes = [0u8; 16];
        temp.copy_from_slice(&mut bytes);
        let rhs_integer = i128::from_le_bytes(bytes);

        let mut rhs_number: Vec<i128> = vec![rhs_integer];
        for value in rhs.decimal.0.iter() {
            rhs_number.push(*value as i128);
        }

        // convert self_number into String
        let mut self_number_string: String = String::new();
        for value in self_number.iter() {
            self_number_string.push_str(&value.to_string());
        }

        // convert rhs_number into String
        let mut rhs_number_string: String = String::new();
        for value in rhs_number.iter() {
            rhs_number_string.push_str(&value.to_string());
        }

        // multiply 2 BigNumber to have the result
        let mut result: BigNumber =
            BigNumber::new(&self_number_string) * BigNumber::new(&rhs_number_string);

        // add the dot
        let len_result: i128 = result.digits.0.len().try_into().unwrap();
        let mut dot_placement: i128 = len_result - dot_placement;

        let mut integer: i128 = 0;
        let mut decimal: VecU8 = VecU8(vec![0]);

        // if dot placement <= 0 add 0 to result
        if dot_placement <= 0 {
            for _ in 0..(dot_placement * -1) {
                result.digits.0.insert(0, 0);
            }
            dot_placement = 0;
        }

        // split number between integer and decimal part
        for (i, value) in result.digits.0.iter().enumerate() {
            if i < dot_placement as usize {
                integer = integer * 10 + *value as i128;
            } else {
                decimal.0.push(*value);
            }
        }
        decimal.0.remove(0);

        let str_integer = integer.to_string();
        let bignumber_integer = BigNumber::new(&str_integer);

        SmallNumber {
            signe: self.signe == rhs.signe,
            integer: bignumber_integer,
            decimal,
        }
    }
}
