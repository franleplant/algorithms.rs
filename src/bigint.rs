use std::ops::Add;


#[derive(Debug, Clone, PartialEq, Eq)]
struct BigInt {
    number: Vec<usize>,
}


impl BigInt {
    fn new(number: Vec<usize>) -> BigInt {
        BigInt {
            number: number.iter().cloned().rev().collect(),
        }
    }
}


impl Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> BigInt {
        let mut numbers = vec![];
        let mut i = 0;
        let mut carry = 0;
        let z = 0;
        let self_len = self.number.len();
        let other_len = rhs.number.len();
        loop {
            if i >= self_len && i >= other_len {
                break;
            }


            let a = self.number.get(i).unwrap_or(&z);
            let b = rhs.number.get(i).unwrap_or(&z);

            let mut sum = a + b + carry;
            if sum >= 10 {
                carry = sum / 10;
                sum = sum - 10;
            }

            println!("a {} b {} sum {} carry {}", a,b,sum,carry);
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
    fn it_works() {
        let n1 = BigInt::new(vec![1,9]);
        let n2 = BigInt::new(vec![1]);
        let n3 = n1 + n2;

        assert_eq!(n3, BigInt::new(vec![2, 0]))
    }
}
