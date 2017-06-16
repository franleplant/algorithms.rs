use std::fmt::Debug;
use std::cmp::PartialOrd;


pub fn merge<T>(arr: &mut [T])
    where T: Debug + PartialOrd + Copy
{
    let len = arr.len();
    let mid = len / 2;
    let (left, right) = {
        let (left, right) = arr.split_at(mid);
        (left.to_vec(), right.to_vec())
    };
    let mut i = 0;
    let mut j = 0;
    let left_len = left.len();
    let right_len = right.len();
    for k in 0..len {
        let left_is_lowest = match (i < left_len, j < right_len) {
            (true, true) => if left[i] <= right[j] { true } else { false },
            (true, false) => true,
            (false, true) => false,
            _ => panic!("can't ever happen"),
        };

        if left_is_lowest {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
    }
}

pub fn merge_sort<T>(arr: &mut [T])
    where T: Debug + PartialOrd + Copy
{
    println!("\nINPUT {:?}", arr);
    if arr.len() == 1 {
        return;
    }

    println!("arr {:?} ", arr);
    let mid = arr.len() / 2;
    {
        let (left, right) = arr.split_at_mut(mid);
        println!("sort1 left {:?} right {:?}", left, right);
        merge_sort(left);
        merge_sort(right);
        println!("sort2 left {:?} right {:?}", left, right);
    }
    println!("merge1 res {:?} ", arr);
    merge(arr);
    println!("merge2 res {:?} ", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_test() {
        let mut arr = vec![1, 2, 4];
        merge(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 4]);

        let mut arr = vec![4, 1, 2];
        merge(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 4]);

        let mut arr = vec![5, 6, 7, 8, 1, 2, 3, 4];
        merge(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        merge(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sort_test() {
        let mut arr = vec![1];
        merge_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1]);

        let mut arr = vec![2, 1];
        merge_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2]);

        let mut arr = vec![4, 2, 1];
        merge_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 4]);

        let mut arr = vec![5, 2, 4, 7, 1, 3, 6];
        merge_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7]);

        let mut arr = vec![5, 2, 4, 1, 3, 6];
        merge_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
