use std::fmt;
use std::ops::{Add, Sub};

pub struct VecU8(pub Vec<u8>);

// On ajouter fmt::Display pour un notre vecteur (VecU8)
impl fmt::Display for VecU8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut number: String = String::new();

        for value in self.0.iter() {
            let digit: u8 = *value;

            number.push_str(&digit.to_string())
        }

        write!(f, "{}", number)
    }
}

pub struct BigNumber {
    pub digits: VecU8,
    pub signe: bool,
}

// le clone
impl Clone for BigNumber {
    fn clone(&self) -> Self {
        let cloned_digits = self.digits.0.iter().cloned().collect();
        let signe: bool = self.signe;
        return BigNumber { digits :  VecU8(cloned_digits), signe};
    }
}

// Le display
impl fmt::Display for BigNumber{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.digits)
    }
}

// Creation du nombre, zéro, un
impl BigNumber {
    #[allow(dead_code)]
    pub fn new(number: &str) -> BigNumber {
        let digits: Vec<u8> = number
            .chars()
            .filter_map(|c: char| c.to_digit(10).map(|d: u32| d as u8))
            .collect();

        BigNumber { digits: VecU8(digits), signe: true }
    }

    #[allow(dead_code)]
    pub fn zero() -> BigNumber {
        BigNumber::new("0")
    }

    #[allow(dead_code)]
    pub fn one() -> BigNumber {
        BigNumber::new("1")
    }
}

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

        BigNumber { digits: VecU8(result), signe: true }
    }
}


impl Sub for BigNumber {
    type Output = BigNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut num1: Vec<u8> = self.digits.0.clone();
        let mut num2: Vec<u8> = rhs.digits.0.clone();

        // Add leading zeros to align the digits
        let difference: usize = num1.len().max(num2.len()) - num1.len();
        for _ in 0..difference {
            num1.insert(0, 0);
        }
        let difference: usize = num1.len().max(num2.len()) - num2.len();
        for _ in 0..difference {
            num2.insert(0, 0);
        }

        let mut result: Vec<u8> = Vec::new();
        let mut borrow: i16 = 0;

        // Subtraction processing
        for (digit1, digit2) in num1.iter().rev().zip(num2.iter().rev()) {
            let diff: i16 = (*digit1 as i16) - (*digit2 as i16) - borrow;
            if diff < 0 {
                borrow = 1;
                result.push((diff + 10) as u8);
            } else {
                borrow = 0;
                result.push(diff as u8);
            }
        }

        // Check if the result is negative and update the sign
        let is_negative: bool = borrow == 1;
        if is_negative {
            for digit in result.iter_mut() {
                *digit = 9 - *digit;
            }
            if let Some(first_mut) = result.first_mut() {
                *first_mut += 1;
            }
        }

        result.reverse();

        BigNumber { digits: VecU8(result), signe: !is_negative }
    }
}
