use std::fmt::Debug;
use std::cmp::PartialOrd;

fn max_heapify<T>(heap: &mut [T], index: usize)
    where T: Debug + PartialOrd + Copy
{
    println!("max_heapify({:?}, {})", heap, index);
    if heap.len() <= 1 {
        return;
    }

    let left = ((index + 1) * 2) - 1;
    let right = ((index + 1) * 2) + 1 - 1;
    let mut largest = index;

    if left < heap.len() && heap[left] > heap[index] {
        largest = left;
    }

    if right < heap.len() && heap[right] > heap[largest] {
        largest = right;
    }

    if largest != index {
        heap.swap(index, largest);
        max_heapify(heap, largest);
    }
}

fn build_max_heap<T>(heap: &mut [T])
    where T: Debug + PartialOrd + Copy
{
    println!("build_max_heap({:?})", heap);
    let len = heap.len();
    let mid = len / 2;
    println!("mid {}", mid);

    for i in (0..mid).rev() {
        println!("i {}", i);
        max_heapify(heap, i);
    }
}

pub fn heap_sort<T>(arr: &mut [T]) -> Vec<T>
    where T: Debug + PartialOrd + Copy
{
    build_max_heap(arr);
    //println!("max heap {:?}", arr);
    let mut v = vec![];
    let mut len = arr.len();

    for _ in (0..len).rev() {
        v.push(arr[0]);
        if len == 1 {
            break;
        }
        arr.swap(0, len - 1);
        //println!("heap sort i {}, {:?} {:?}", i, v, arr);
        len -= 1;
        max_heapify(&mut arr[..len], 0);
        //println!("max_heapifiED {:?}", &arr[..len]);
    }

    v
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_heapify_test() {
        let mut heap = vec![2, 4, 1];
        max_heapify(heap.as_mut_slice(), 0);
        assert_eq!(heap, vec![4, 2, 1]);

        let mut heap = vec![16, 4, 10, 14, 7, 9, 3, 2, 8, 1];
        max_heapify(heap.as_mut_slice(), 1);
        assert_eq!(heap, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }

    #[test]
    fn build_max_heap_test() {
        let mut heap = vec![1, 2, 4];
        build_max_heap(heap.as_mut_slice());
        assert_eq!(heap, vec![4, 2, 1]);

        let mut heap = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        build_max_heap(heap.as_mut_slice());
        assert_eq!(heap, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1], "case 1");
    }

    #[test]
    fn heap_sort_test() {
        let mut arr = vec![1, 2, 4];
        let sorted = heap_sort(arr.as_mut_slice());
        assert_eq!(sorted, vec![4, 2, 1]);

        let mut arr = vec![1, 16, 4, 10, 14, 7, 9, 3, 2, 8];
        let sorted = heap_sort(arr.as_mut_slice());
        assert_eq!(sorted, vec![16, 14, 10, 9, 8, 7, 4, 3, 2, 1]);
    }
}
