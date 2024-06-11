use crate::bignumber::bignumber::BigNumber;

pub struct BigNumberIterator {
    number: BigNumber,
    index: BigNumber,
}

impl Iterator for BigNumberIterator {
    type Item = BigNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.clone() < self.number {
            let current_index: BigNumber = self.index.clone();
            self.index = self.index.clone() + BigNumber::one();
            Some(current_index)
        } else {
            None
        }
    }
}

impl IntoIterator for BigNumber {
    type Item = BigNumber;
    type IntoIter = BigNumberIterator;

    fn into_iter(self) -> Self::IntoIter {
        BigNumberIterator {
            number: self,
            index: BigNumber::zero(),
        }
    }
}
