use super::bignumber::BigNumber;
use std::fmt;
use std::ops::Add;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct SmallNumber {
    pub signe: bool,
    pub integer: i128,
    pub decimal: VecU8,
}

// Le display
impl fmt::Display for SmallNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.signe {
            write!(f, "{}.{}", self.integer, self.decimal)
        } else {
            if self.integer < 0 {
                write!(f, "-{}.{}", -self.integer, self.decimal)
            } else {
                write!(f, "-{}.{}", self.integer, self.decimal)
            }
        }
    }
}

// implementation new
impl SmallNumber {
    pub fn new(number: &str) -> SmallNumber {
        let parts: Vec<&str> = number.split(".").collect();

        if parts.len() == 2 {
            let integer: i128 = parts[0].parse().unwrap();
            let decimal: Vec<u8> = parts[1]
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();

            if number.contains('-') {
                SmallNumber {
                    signe: false,
                    integer,
                    decimal: VecU8(decimal),
                }
            } else {
                SmallNumber {
                    signe: true,
                    integer,
                    decimal: VecU8(decimal),
                }
            }
        } else {
            panic!("Invalid format");
        }
    }
}

impl Add for SmallNumber {
    type Output = SmallNumber;

    fn add(self, rhs: Self) -> Self::Output {
        // if the two decimal parts of the numbers have not the same length
        // then we add zero to the rigne to the smallest number to have the same length
        let mut lhs_decimal: Vec<u8> = self.decimal.0;
        let mut rhs_decimal: Vec<u8> = rhs.decimal.0;

        let mut difference: usize = lhs_decimal.len().abs_diff(rhs_decimal.len());
        
        // here the 0 are added if the two decimal parts are not the same
        if lhs_decimal.len() == rhs_decimal.len() {
            difference = lhs_decimal.len();
        } else if lhs_decimal.len() > rhs_decimal.len() {
            for _ in 0..difference {
                rhs_decimal.push(0);
            }
            difference = lhs_decimal.len();
        } else if lhs_decimal.len() < rhs_decimal.len() {
            for _ in 0..difference {
                lhs_decimal.push(0);
            }
            difference = rhs_decimal.len()
        }

        // convert lhs_decimal to &str and then create a variable self_decimal_number
        // and store a big number with the value of the string
        let self_decimal_number_str: String = lhs_decimal
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        let self_decimal_number: BigNumber = BigNumber::new(&self_decimal_number_str);

        // Same thing for rhs_decimal
        let rhs_decimal_number_str: String = rhs_decimal
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        let rhs_decimal_number: BigNumber = BigNumber::new(&rhs_decimal_number_str);

        // create a big number with 10^difference from string
        let ten_pow_difference: String = format!("1{}", "0".repeat(difference));
        let mut one_ten_pow_difference: BigNumber = BigNumber::new(&ten_pow_difference);

        // if pos-pos or neg-neg
        if (self.signe && rhs.signe) || (!self.signe && !rhs.signe){
            // first we add the two integer parts
            let mut integer: i128 = self.integer + rhs.integer;

            // adding the two decimal part
            let decimal_addition: BigNumber = self_decimal_number + rhs_decimal_number;

            println!("{} et {}", decimal_addition, one_ten_pow_difference);

            // substract to verify if < 0
            // verify the carry
            one_ten_pow_difference = one_ten_pow_difference - decimal_addition.clone();
            if !one_ten_pow_difference.signe || one_ten_pow_difference.is_zero() {
                integer = if self.signe && rhs.signe {
                    integer + 1
                } else {
                    integer - 1
                };
                one_ten_pow_difference.delete_first_digit();
            } else {
                one_ten_pow_difference = decimal_addition;
            };

            // we create decimal and store the result here
            let decimal: Vec<u8> = one_ten_pow_difference.digits.0;

            // change signe if neg-neg
            let new_signe = if self.signe && rhs.signe {
                true
            } else {
                false
            };

            SmallNumber {
                integer: integer,
                signe: new_signe,
                decimal: VecU8(decimal),
            }
        } else {
            unimplemented!()
        }
    }
}
