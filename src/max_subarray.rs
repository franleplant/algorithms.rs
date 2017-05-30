
// TODO accept any type of number ?
fn find_max_crossing_subarray(arr: &[f64]) -> (usize, usize, f64) {
    let len = arr.len();
    let low = 0;
    let high = len - 1;
    let mid: f64 = (high - low) as f64 / 2f64;
    let low_mid = mid.floor() as usize;
    let high_mid = mid.ceil() as usize;

    // println!("len {} mid: {}, low: ({}, {}), high: ({}, {})",
    // len, mid, low, low_mid, high_mid, high);

    let mut left_sum = -::std::f64::INFINITY;
    let mut sum = 0f64;
    let mut max_left = 0;

    for i in (low..low_mid + 1).rev() {
        // println!("{}", i);
        let el = arr[i];
        sum = sum + el;
        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = -::std::f64::INFINITY;
    let mut sum = 0f64;
    let mut max_right = 0;

    for i in high_mid..high + 1 {
        // println!("{}", i);
        let el = arr[i];
        sum = sum + el;
        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    (max_left, max_right, left_sum + right_sum)
}


pub fn find_max_subarray(arr: &[f64]) -> (usize, usize, f64) {
    let len = arr.len();
    let low = 0;
    let high = len - 1;

    if high == low {
        return (low, high, arr[low]);
    }

    let mid: f64 = (high - low) as f64 / 2f64;
    let low_mid = mid.floor() as usize;
    let high_mid = mid.ceil() as usize;

    let (left_low, left_high, left_sum) = find_max_subarray(&arr[low..low_mid + 1]);
    let (right_low, right_high, right_sum) = find_max_subarray(&arr[high_mid..high + 1]);
    let (cross_low, cross_high, cross_sum) = find_max_crossing_subarray(&arr[..]);


    if left_sum >= right_sum && left_sum >= cross_sum {
        (left_low, left_high, left_sum)
    } else if right_sum >= left_sum && right_sum >= cross_sum {
        (right_low, right_high, right_sum)
    } else {
        (cross_low, cross_high, cross_sum)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: Vec<f64> = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4,
                                   7]
                .iter()
                .map(|&x| x as f64)
                .collect();
        let result = find_max_crossing_subarray(&input[..]);
        println!("{:?}", result);
        assert_eq!(result, (7, 10, 43f64));

        let result = find_max_subarray(&input[..]);
        let &(low, high, _) = &result;
        println!("max sub {:?}, {:?}", result, &input[low..high]);
        assert_eq!(result, (7, 10, 43f64));
    }
}

// extern crate test;
// mod bench {
// use super::*;

// #[bench]
// fn bench_1(b: &mut test::Bencher) {
// let input: Vec<f64> = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4,
// 7]
// .iter()
// .map(|&x| x as f64)
// .collect();

// b.iter(|| find_max_subarray(&input[..]));
// }
// }
