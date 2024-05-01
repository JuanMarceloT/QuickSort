

fn lomuto_partition(arr: &mut [i16], low: usize, high: usize) -> usize{
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn quicksort(arr: &mut [i16], low: usize, high: usize){
    if low < high {
        let partition_index = lomuto_partition(arr, low, high);
        //println!("{:?} and {:?}", arr, partition_index);
        quicksort(arr, low, partition_index.checked_sub(1).unwrap_or(low));
        quicksort(arr, partition_index.checked_add(1).unwrap_or(high), high);
    }
}




fn main() {
    let mut arr = [8, 7, 2,15,5,02,48,9898,975,545];
    let arr_length = arr.len();
    println!("{:?}", arr);
    quicksort(&mut arr, 0, arr_length - 1);
    println!("{:?}", arr);


}



#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_quicksort() {
        let mut arr1 = [8, 7, 2, 15, 5, 2, 48, 9898, 975, 545];
        let arr1_length = arr1.len();
        quicksort(&mut arr1, 0, arr1_length - 1);
        assert_eq!(arr1, [2, 2, 5, 7, 8, 15, 48, 545, 975, 9898]);

        let mut arr2 = [5, 4, 3, 2, 1];
        let arr2_length = arr2.len();
        quicksort(&mut arr2, 0, arr2_length - 1);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        let mut arr3 = [1, 3, 5, 7, 9];
        let arr3_length = arr3.len();
        quicksort(&mut arr3, 0, arr3_length - 1);
        assert_eq!(arr3, [1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_quicksort_with_rand_nums() {
        for _ in 0..20 {
            const LENGTH :usize = 100 * 1000;
            let mut rng = rand::thread_rng();
            let mut arr = [0; LENGTH];

            for i in 0..LENGTH {
                arr[i] = rng.gen_range(0..1000);
            }
            let arr_length = arr.len();
            quicksort(&mut arr, 0, arr_length - 1);

            for i in 1..LENGTH {
                assert!(arr[i - 1] <= arr[i]);
            }
        }
    }
}