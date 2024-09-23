use std::cmp::PartialEq;
use std::fmt;
use super::super::bignumber::bignumber::BigNumber;

pub struct VecU8(pub Vec<u8>);

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

// impl clone for VecU8
impl Clone for VecU8 {
    fn clone(&self) -> Self {
        let cloned_digits: Vec<u8> = self.0.iter().cloned().collect();
        VecU8(cloned_digits)
    }
}

// impl PartialEq for VecU8
impl PartialEq for VecU8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct SmallNumber {
    pub signe: bool,
    pub integer: BigNumber,
    pub decimal: VecU8,
}

// impl .clone for SmallNumber
impl Clone for SmallNumber {
    fn clone(&self) -> Self {
        let signe: bool = self.signe;
        let integer: BigNumber = self.integer.clone();
        let decimal: VecU8 = self.decimal.clone();

        SmallNumber {
            signe,
            integer,
            decimal,
        }
    }
}

// Le display
impl fmt::Display for SmallNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.signe {
            write!(f, "{}.{}", self.integer, self.decimal)
        } else {
            if self.integer < BigNumber::zero() {
                write!(f, "{}.{}", self.integer, self.decimal)
            } else {
                write!(f, "-{}.{}", self.integer, self.decimal)
            }
        }
    }
}

// implementation new
impl SmallNumber {
    pub fn new(number: &str) -> SmallNumber {
        let mut parts: Vec<&str> = number.split(".").collect();

        if parts.len() == 2 {
            // remove useless zero
            while parts[1].ends_with('0') {
                parts[1] = &parts[1][..parts[1].len() - 1];
            }
            // if empty add 1 zero
            if parts[1].is_empty() {
                parts[1] = "0";
            }

            let integer: BigNumber = BigNumber::new(parts[0]);
            let decimal: Vec<u8> = parts[1]
                .chars()
                .map(|c: char| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();

            if number.contains('-') {
                let mut result = SmallNumber {
                    signe: false,
                    integer: integer,
                    decimal: VecU8(decimal),
                };
                result.remove_zeros_decimal();
                result
            } else {
                let mut result = SmallNumber {
                    signe: true,
                    integer,
                    decimal: VecU8(decimal),
                };
                result.remove_zeros_decimal();
                result
            }
        } else {
            panic!("Invalid format (number.decimal needed)");
        }
    }

    pub fn len(&self) -> BigNumber {
        let mut len = self.integer.len();
        for _i in &self.decimal.0 {
            len = len + BigNumber::one()
        }
        len
    }

    pub fn len_decimal(&self) -> BigNumber {
        let mut len = BigNumber::zero();
        for _i in &self.decimal.0 {
            len = len + BigNumber::one()
        }
        len
    }

    pub fn remove_zeros_decimal(&mut self) {
        while self.decimal.0.last() == Some(&0) {
            self.decimal.0.pop();
        }
        if self.len_decimal() == BigNumber::zero() {
            self.decimal.0.push(0);
        }
    }

    pub fn div(&self, other: &SmallNumber) -> SmallNumber {
        // full div for 2 small numbers
        SmallNumber::new("2.0")
    }

    
}

// impl PartialEq
impl PartialEq for SmallNumber {
    fn eq(&self, other: &Self) -> bool {
        self.signe == other.signe && self.integer == other.integer && self.decimal == other.decimal
    }
}
