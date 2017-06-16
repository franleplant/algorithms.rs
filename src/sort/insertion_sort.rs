use std::fmt::Debug;
use std::cmp::PartialOrd;


//TODO how to output all the printlns conditonally? Useful for benchmarks and stuff

pub fn insertion_sort<T>(arr: &mut [T])
    where T: Debug + PartialOrd + Copy
{
    println!("\nSTART\n");
    println!("INPUT = {:?}", arr);
    let len = arr.len();
    for j in 1..len {
        println!("\nITERATION = {}\n", j);
        println!("arr {:?}", arr);
        println!("\nvalue = {:?}", arr[j]);
        let value = arr[j];

        for i in (0..j).rev() {
            println!("arr[{}] = {:?}", i, arr[i]);
            if arr[i] <= value {
                println!("break!");
                println!("arr {:?}", arr);
                break;
            }

            arr.swap(i, i + 1);
            println!("swap!");
            println!("arr {:?}", arr);
        }
    }

    println!("\nEND\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let mut arr = vec![4, 2, 1];
        insertion_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 4]);

        let mut arr = vec![5, 2, 4, 6, 1, 3];
        insertion_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
