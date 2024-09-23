use super::super::super::bignumber::bignumber::{BigNumber, VecU8};
use std::ops::Sub;

impl Sub for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut num1: Vec<u8> = self.digits.0;
        let mut num2: Vec<u8> = rhs.digits.0;

        // Add some zero to have the same len
        let difference: usize = (num1.len() as isize - num2.len() as isize).abs() as usize;
        if num1.len() < num2.len() {
            for _ in 0..difference {
                num1.insert(0, 0);
            }
        } else if num1.len() > num2.len() {
            for _ in 0..difference {
                num2.insert(0, 0);
            }
        }

        // Result and carry
        let mut result: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;

        // Sub processing
        for (digit1, digit2) in num1.iter().rev().zip(num2.iter().rev()) {
            let diff: i16 = (*digit1 as i16) - (*digit2 as i16) - carry as i16;
            if diff < 0 {
                carry = 1;
                result.push((diff + 10) as u8);
            } else {
                carry = 0;
                result.push(diff as u8);
            }
        }

        // Check if the result is negative and update the sign
        let is_negative: bool = carry == 1;
        if is_negative {
            for digit in result.iter_mut() {
                *digit = 9 - *digit;
            }
            if let Some(first_mut) = result.first_mut() {
                *first_mut += 1;
            }
        }

        let len_result: usize = result.len();

        result.reverse();

        // add 0 if result is empty
        if result.len() == 0 {
            result.push(0);
        }

        // if the last element >= 10 in result, then we add 1 the the second to last and remove the last value
        if result[len_result - 1] >= 10 {
            result[len_result - 2] += 1;
            result.remove(len_result - 1);
        }

        BigNumber {
            digits: VecU8(result),
            signe: !is_negative,
            division_precision: self.division_precision,
        }
    }
}