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
        write!(f, "{}.{}", self.integer, self.decimal)
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
        // Dec des nombres
        let mut dec1: Vec<u8> = self.decimal.0;
        let mut dec2: Vec<u8> = rhs.decimal.0;

        // dec la plus grande
        let mut max_digits: usize = 0;

        // add some zeros
        let difference: usize = (dec1.len() as isize - dec2.len() as isize).abs() as usize;
        if dec1.len() < dec2.len() {
            for _ in 0..difference {
                dec1.push(0);
            }
            max_digits = dec2.len();
        } else if dec1.len() > dec2.len() {
            for _ in 0..difference {
                dec2.push(0);
            }
            max_digits = dec1.len();
        };

        let mut dec_result: Vec<u8> = Vec::new();
        let mut carry: u8 = 0;

        if (self.signe && rhs.signe) || (!self.signe && !rhs.signe) {
            // resultat des entiers
            let mut int_result: i128 = self.integer + rhs.integer;

            // Add processing
            for (digit1, digit2) in dec1.iter().rev().zip(dec2.iter().rev()) {
                let sum: u8 = *digit1 + digit2 + carry;
                dec_result.push(sum % 10);
                carry = sum / 10;
            }

            if carry > 0 {
                dec_result.push(carry);
            }

            if self.signe && rhs.signe {
                if dec_result.len() > max_digits {
                    dec_result.reverse();

                    dec_result.remove(0);
                    int_result += 1;

                    SmallNumber {
                        signe: true,
                        integer: int_result,
                        decimal: VecU8(dec_result),
                    }
                } else {
                    SmallNumber {
                        signe: true,
                        integer: int_result,
                        decimal: VecU8(dec_result),
                    }
                }
            } else {
                if dec_result.len() > max_digits {
                    dec_result.reverse();

                    dec_result.remove(0);
                    int_result += 1;

                    // on change le signe
                    int_result = -int_result;

                    SmallNumber {
                        signe: false,
                        integer: int_result,
                        decimal: VecU8(dec_result),
                    }
                } else {
                    int_result = -int_result;
                    SmallNumber {
                        signe: false,
                        integer: int_result,
                        decimal: VecU8(dec_result),
                    }
                }
            }
        } else {
            if self.integer.abs() > rhs.integer.abs() || self.integer.abs() == rhs.integer.abs() {
                println!("Self est plus grand que rhs ou les meme integer");

                let str1: String = dec1.iter().map(|x: &u8| x.to_string()).collect::<String>();
                let str2: String = dec2.iter().map(|x: &u8| x.to_string()).collect::<String>();

                // On les convertit en grand nombre
                let num_dec1: BigNumber = BigNumber::new(&str1);
                let num_dec2: BigNumber = BigNumber::new(&str2);

                // on fait la difference
                let difference: BigNumber = num_dec1 - num_dec2;

                // si le resultat est negatif on ajoute la carry
                if !difference.signe {
                    let mut max_number: String = String::from("1");
                    for _ in 0..max_digits {
                        max_number.push('0');
                    }

                    let mut max_number: BigNumber = BigNumber::new(&max_number);

                    let sub_bumber: BigNumber = BigNumber::new(&str2);
                    let add_number: BigNumber = BigNumber::new(&str1);

                    max_number = max_number + add_number;

                    let difference: BigNumber = max_number - sub_bumber;

                    // resultat
                    let mut dec: Vec<u8> = difference.digits.0;

                    // resultat int
                    let mut int_result: i128 = self.integer.abs() - rhs.integer.abs();

                    // on check le final
                    if dec.len() < max_digits {
                        if !(int_result == 0) {
                            int_result += 1;
                        }
                    }

                    dec.remove(0);

                    println!("{} et {:?}", int_result, dec);
                }

                SmallNumber::new("0.0")
            } else {
                SmallNumber::new("0.0")
            }
        }
    }
}
