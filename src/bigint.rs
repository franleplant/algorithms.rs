use std::ops::{Add, Sub};


#[derive(Debug, Clone)]
struct BigInt {
    number: Vec<usize>,
}


impl BigInt {
    pub fn new(number: Vec<usize>) -> BigInt {
        BigInt { number: number }
    }

    pub fn get_digit(&self, index: usize) -> usize {
        let len = self.number.len();
        if index > len - 1 {
            return 0;
        }

        self.number[len - index - 1]
    }

    pub fn get_len(&self) -> usize {
        self.number.len()
    }
}


impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        let self_len = self.get_len();
        let other_len = other.get_len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut equality = true;
        for i in 0..max_len {
            let digit_equality = self.get_digit(i) == other.get_digit(i);
            equality = equality && digit_equality;
        }

        equality
    }
}


impl Add for BigInt {
    type Output = BigInt;
    fn add(self, other: BigInt) -> BigInt {
        let self_len = self.get_len();
        let other_len = other.get_len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut carry = 0;
        let mut numbers = vec![];

        // at most we can end up with a len + 1 bigint
        for i in 0..(max_len + 1) {
            let a = self.get_digit(i);
            let b = other.get_digit(i);

            let mut res = a + b + carry;
            if res >= 10 {
                carry = res / 10;
                res = res - 10;
            }

            //println!("a {} b {} res {} carry {}", a,b,res,carry);
            numbers.push(res);
        }

        BigInt::new(numbers.iter().cloned().rev().collect())
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, other: BigInt) -> BigInt {
        let self_len = self.number.len();
        let other_len = other.number.len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut carry = 0;
        let mut numbers = vec![];

        for i in 0..max_len {
            let a = self.get_digit(i);
            let b = other.get_digit(i);
            //println!("1) {} - {} - {}", a,b,carry);

            let res = if b + carry > a {
                let res = a + 10 - b - carry;
                carry = 1;
                res
            } else {
                let res = a - b - carry;
                carry = 0;
                res
            };

            //println!("2) = {}  with carry {}", res,carry);
            numbers.push(res);
        }

        BigInt::new(numbers.iter().cloned().rev().collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(BigInt::new(vec![1]), BigInt::new(vec![1]));
        assert_eq!(BigInt::new(vec![1, 2, 3]), BigInt::new(vec![1, 2, 3]));
        assert_eq!(BigInt::new(vec![0, 0]), BigInt::new(vec![0, 0, 0, 0]));
    }

    #[test]
    fn add() {
        let n1 = BigInt::new(vec![9, 9]);
        let n2 = BigInt::new(vec![1, 0]);
        assert_eq!(n1 + n2, BigInt::new(vec![1, 0, 9]))
    }

    #[test]
    fn sub() {
        let n1 = BigInt::new(vec![1, 0, 0]);
        let n2 = BigInt::new(vec![1]);
        assert_eq!(n1 - n2, BigInt::new(vec![9, 9]))
    }
}
