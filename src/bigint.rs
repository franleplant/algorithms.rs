use std::ops::{Add, Sub};

static CHUNK_SIZE: usize = 10;

#[derive(Debug, Clone)]
pub struct BigUInt {
    chunks: Vec<usize>,
    chunk_size: usize,
    chunk_max: usize,
}


impl BigUInt {
    pub fn new(digits: &[usize]) -> BigUInt {
        //println!("new input {:?}", digits);
        let mut chunks = vec![0];
        let mut chunk_index = 0;

        for (i, digit) in digits.iter().rev().enumerate() {
            if i != 0 && i % CHUNK_SIZE == 0 {
                chunk_index += 1;
                chunks.push(0);
            }

            //println!("{:?}", chunks);
            chunks[chunk_index] = chunks[chunk_index] +
                                  digit * 10usize.pow((i % CHUNK_SIZE) as u32);
        }

        BigUInt {
            chunks: chunks,
            chunk_size: CHUNK_SIZE,
            chunk_max: 10usize.pow(CHUNK_SIZE as u32),
        }
    }

    pub fn from_chunks(chunks: Vec<usize>) -> BigUInt {
        BigUInt {
            chunks: chunks,
            chunk_size: CHUNK_SIZE,
            chunk_max: 10usize.pow(CHUNK_SIZE as u32),
        }
    }

    pub fn get_chunk(&self, index: usize) -> usize {
        let z = 0;
        self.chunks.get(index).unwrap_or(&z).clone()
    }

    pub fn get_len(&self) -> usize {
        self.chunks.len()
    }
}


impl PartialEq for BigUInt {
    fn eq(&self, other: &BigUInt) -> bool {
        let self_len = self.get_len();
        let other_len = other.get_len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut equality = true;
        for i in 0..max_len {
            let chunk_equality = self.get_chunk(i) == other.get_chunk(i);
            equality = equality && chunk_equality;
            if !equality {
                break;
            }
        }

        equality
    }
}


impl Add for BigUInt {
    type Output = BigUInt;
    fn add(self, other: BigUInt) -> BigUInt {
        let self_len = self.get_len();
        let other_len = other.get_len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut carry = 0;
        let mut chunks = vec![];

        // at most we can end up with a len + 1 bigint
        for i in 0..(max_len + 1) {
            let a = self.get_chunk(i);
            let b = other.get_chunk(i);

            let mut res = a + b + carry;
            //println!("1) a{} + b{} + c{} = r{}", a,b,carry,res);
            if res >= self.chunk_max {
                carry = res / self.chunk_max;
                res = res - self.chunk_max;
            } else {
                carry = 0;
            }

            //println!("2) a{} + b{} + c{} = r{}", a,b,carry,res);
            chunks.push(res);
        }

        BigUInt::from_chunks(chunks)
    }
}

impl Sub for BigUInt {
    type Output = BigUInt;
    fn sub(self, other: BigUInt) -> BigUInt {
        // TODO compare both and if other is bigger (need to implement partialOrd)
        // then return 0;
        let self_len = self.get_len();
        let other_len = other.get_len();
        let max_len = if self_len >= other_len {
            self_len
        } else {
            other_len
        };

        let mut carry = 0;
        let mut chunks = vec![];

        for i in 0..max_len {
            let a = self.get_chunk(i);
            let b = other.get_chunk(i);
            //println!("1) {} - {} - {}", a,b,carry);

            let res = if b + carry > a {
                let res = a + self.chunk_max - b - carry;
                carry = 1;
                res
            } else {
                let res = a - b - carry;
                carry = 0;
                res
            };

            //println!("2) = {}  with carry {}", res,carry);
            chunks.push(res);
        }

        BigUInt::from_chunks(chunks)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let n = BigUInt::new(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        println!("BIGUINT {:?}", n);
        assert_eq!(n.chunks, vec![1_000_000_000, 2_000_000_000]);
        assert_eq!(n, BigUInt::from_chunks(vec![1_000_000_000, 2_000_000_000]));

        let n = BigUInt::new(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        assert_eq!(n.chunks, vec![9_999_999_999, 999_999_999]);
        assert_eq!(n, BigUInt::from_chunks(vec![9_999_999_999, 999_999_999]));
    }

    #[test]
    fn equality() {
        assert_eq!(BigUInt::new(&[1]), BigUInt::new(&[1]));
        assert_eq!(BigUInt::new(&[1, 2, 3]), BigUInt::new(&[1, 2, 3]));
        assert_eq!(BigUInt::new(&[0, 0]), BigUInt::new(&[0, 0, 0, 0]));
    }

    #[test]
    fn add() {
        let n1 = BigUInt::new(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
        let n2 = BigUInt::new(&[1]);
        assert_eq!(n1 + n2,
                   BigUInt::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))
    }

    #[test]
    fn sub1() {
        let n1 = BigUInt::new(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let n2 = BigUInt::new(&[1]);
        assert_eq!(n1 - n2,
                   BigUInt::new(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]))
    }
}
