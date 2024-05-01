

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
       // println!("{:?} and {:?}", arr, partition_index);
        quicksort(arr, low, partition_index - 1);
        quicksort(arr, partition_index + 1, high);
    }
}


fn main() {
    let mut arr = [8, 7, 2,15,5,02,48,9898,975,545];
    let arr_length = arr.len();
    println!("{:?}", arr);
    quicksort(&mut arr, 0, arr_length - 1);
    println!("{:?}", arr);


}
