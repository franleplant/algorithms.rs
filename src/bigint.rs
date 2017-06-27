use std::ops::Add;


#[derive(Debug, Clone, PartialEq, Eq)]
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
}


impl Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        let self_len = self.number.len();
        let other_len = rhs.number.len();

        let mut i = 0;
        let mut carry = 0;
        let mut numbers = vec![];
        loop {
            if i > self_len && i > other_len {
                break;
            }

            let a = self.get_digit(i);
            let b = rhs.get_digit(i);

            let mut sum = a + b + carry;
            if sum >= 10 {
                carry = sum / 10;
                sum = sum - 10;
            }

            println!("a {} b {} sum {} carry {}", a, b, sum, carry);
            numbers.push(sum);
            i += 1;
        }

        BigInt::new(numbers.iter().cloned().rev().collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let n1 = BigInt::new(vec![9, 9]);
        let n2 = BigInt::new(vec![1, 0]);
        let n3 = n1 + n2;

        assert_eq!(n3, BigInt::new(vec![1, 0, 9]))
    }
}
