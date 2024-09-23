use super::super::super::bignumber::bignumber::{BigNumber, VecU8};
use std::ops::Add;

impl Add for BigNumber {
    type Output = BigNumber;

    fn add(self, rhs: Self) -> Self::Output {
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

        // Add processing
        for (digit1, digit2) in num1.iter().rev().zip(num2.iter().rev()) {
            let sum: u8 = *digit1 + digit2 + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        result.reverse();

        BigNumber {
            digits: VecU8(result),
            signe: true,
            division_precision: self.division_precision,
        }
    }
}