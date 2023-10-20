use std::cmp::{PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Sub, Mul};

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
        return BigNumber {
            digits: VecU8(cloned_digits),
            signe,
        };
    }
}

// Le display
impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.signe {
            write!(f, "{}", self.digits)
        } else {
            write!(f, "-{}", self.digits)
        }
    }
}

// Creation du nombre, zéro, un
impl BigNumber {
    #[allow(dead_code)]
    pub fn new(number: &str) -> BigNumber {
        let mut digits: Vec<u8> = number
            .chars()
            .filter_map(|c: char| c.to_digit(10).map(|d: u32| d as u8))
            .collect();

        // remove the first zeros if there are in digits
        // example : [0, 0, 4, 6] -> [4, 6]
        digits = digits.into_iter().skip_while(|&x| x == 0).collect();

        // if the number is negative, we change the signe
        let signe: bool = !number.starts_with('-');

        BigNumber {
            digits: VecU8(digits),
            signe: signe,
        }
    }

    #[allow(dead_code)]
    pub fn zero() -> BigNumber {
        BigNumber::new("0")
    }

    #[allow(dead_code)]
    pub fn one() -> BigNumber {
        BigNumber::new("1")
    }

    // function to delete the first digit of digits
    pub fn delete_first_digit(&mut self) {
        self.digits.0.remove(0);
    }

    // function to verify if every digit equal 0
    pub fn is_zero(&self) -> bool {
        for digit in self.digits.0.iter() {
            if *digit != 0 {
                return false;
            }
        }
        true
    }
}

// impl of PartialEq
impl PartialEq for BigNumber {
    fn eq(&self, other: &Self) -> bool {
        if self.digits.0.len() != other.digits.0.len() {
            return false;
        }
        for (digit1, digit2) in self.digits.0.iter().zip(other.digits.0.iter()) {
            if digit1 != digit2 {
                return false;
            }
        }
        true
    }
}

// impl of PartialOrd just if a BigNumber is greater than the other
impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.digits.0.len() > other.digits.0.len() {
            return Some(std::cmp::Ordering::Greater);
        } else if self.digits.0.len() < other.digits.0.len() {
            return Some(std::cmp::Ordering::Less);
        } else {
            for (digit1, digit2) in self.digits.0.iter().zip(other.digits.0.iter()) {
                if digit1 > digit2 {
                    return Some(std::cmp::Ordering::Greater);
                } else if digit1 < digit2 {
                    return Some(std::cmp::Ordering::Less);
                }
            }
        }
        Some(std::cmp::Ordering::Equal)
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

        BigNumber {
            digits: VecU8(result),
            signe: true,
        }
    }
}

// impl Sub for BigNumber
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
        }
    }
}

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

        // find the signe
        let signe: bool = self.signe == rhs.signe;

        BigNumber {
            digits: VecU8(result_final),
            signe
        }
    }
}
