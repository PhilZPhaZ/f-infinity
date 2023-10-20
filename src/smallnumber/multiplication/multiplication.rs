use super::super::smallnumber::{SmallNumber, VecU8};

impl std::ops::Mul for SmallNumber {
    type Output = SmallNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        // create a Vec i128 with self.integer (i128) an self.decimal.0 (Vec u8)
        let mut self_number: Vec<i128> = vec![self.integer];
        for value in self.decimal.0.iter() {
            self_number.push(*value as i128);
        }

        // same for rhs
        let mut rhs_number: Vec<i128> = vec![rhs.integer];
        for value in rhs.decimal.0.iter() {
            rhs_number.push(*value as i128);
        }

        SmallNumber::new("0.0")
    }
}