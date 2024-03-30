use super::super::super::bignumber::bignumber::{BigNumber, VecU8};
use std::ops::Mul;

impl Mul for BigNumber {
    type Output = BigNumber;

    fn mul(self, rhs: Self) -> Self::Output {
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

        let mut result: Vec<Vec<u8>> = vec![];

        // reverse the list to do the operation
        num1.reverse();
        num2.reverse();

        // loop for the multiplication
        for (i, mul_number_down) in num2.iter().enumerate() {
            let mut carry: u8 = 0;

            let mut sub_result: Vec<u8> = vec![];

            for (_, mul_number_up) in num1.iter().enumerate() {
                let mul: u8 = mul_number_down * mul_number_up + carry;
                sub_result.insert(0, mul % 10);
                carry = mul / 10;
            }
            sub_result.insert(0, carry);

            for _ in 0..i {
                sub_result.push(0);
            }

            result.push(sub_result);
        }
        // reserve to do the addition
        result.iter_mut().for_each(|v| {
            v.reverse();
        });
        // add zeros to vec in result to have the same length
        let max_len: usize = result.iter().map(|v| v.len()).max().unwrap();
        result.iter_mut().for_each(|v| {
            let len: usize = v.len();
            for _ in 0..(max_len - len) {
                v.push(0)
            }
        });

        // final addition of vec in result
        let mut result_final: Vec<u8> = vec![];
        let mut carry: u8 = 0;

        // add first element of each vec in result and if > 10 add to carry and add to next elem of vec
        for i in 0..max_len {
            let mut sum: u8 = 0;
            for j in 0..result.len() {
                sum += result[j][i];
            }
            sum += carry;
            result_final.push(sum % 10);
            carry = sum / 10;
        }

        // reverse the result
        result_final.reverse();

        // remove useless 0
        result_final = result_final.into_iter().skip_while(|&x| x == 0).collect();
        if result_final.is_empty() {
            result_final.push(0);
        }

        // find the signe
        let signe: bool = self.signe == rhs.signe;

        BigNumber {
            digits: VecU8(result_final),
            signe,
        }
    }
}