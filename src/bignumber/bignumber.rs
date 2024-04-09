use std::cmp::{PartialEq, PartialOrd};
use std::fmt;

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
        if digits.is_empty() {
            digits.push(0);
        }

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

    pub fn set_signe_to_positive(&mut self) {
        self.signe = true;
    }

    pub fn len(&self) -> BigNumber {
        let mut len = BigNumber::zero();
        for _i in &self.digits.0 {
            len = len + BigNumber::one()
        }
        len
    }

    pub fn separate_in_two_str(&self, number_of_digits: BigNumber) -> (String, Vec<u8>) {
        let mut i = BigNumber::zero();
        let mut string_result_first_part: String = String::new();
        let mut vec_result_second_part: Vec<u8> = vec![];
        let mut index = 0;

        while i < number_of_digits {
            i = i + BigNumber::one();
            string_result_first_part.push_str(format!("{}", self.digits.0[index]).as_str());
            index += 1;
        }
        while i < self.len() {
            vec_result_second_part.push(self.digits.0[index]);
            i = i + BigNumber::one();
            index += 1;
        }

        (string_result_first_part, vec_result_second_part)
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


