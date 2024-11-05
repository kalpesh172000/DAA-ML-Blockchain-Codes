use rand::Rng;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    let size = 1000;

    let mut arr_deter: Vec<i32> = (0..size).map(|_| rng.gen_range(0..10000)).collect();
    let mut arr_random = arr_deter.clone();

    let len_deter = arr_deter.len() as isize;
    let len_random = arr_deter.len() as isize;

    let start = Instant::now();
    quicksort_deter(&mut arr_deter, 0, len_deter - 1);
    let deter_time = start.elapsed();

    let start = Instant::now();
    quicksort_random(&mut arr_random, 0, len_random - 1);
    let random_time = start.elapsed();


    println!("quicksort using deterministic pivot took {:?} time ", deter_time);
    println!("quicksort using randomized pivot took {:?} time ", random_time);
}

fn quicksort_deter(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition_deter(arr, low, high);
        quicksort_deter(arr, low, p - 1); //assumes p is at its right place
        quicksort_deter(arr, p + 1, high); //assumes p is at its right place
    }
}

fn partition_deter(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize]; //
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize); // i+1 is in bracket
    i + 1
}

fn quicksort_random(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition_random(arr, low, high);
        quicksort_random(arr, low, p - 1); //assumes p is at its right place
        quicksort_random(arr, p + 1, high); //assumes p is at its right place
    }
}

fn partition_random(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot_index = low + rand::thread_rng().gen_range(0..high - low + 1);
    arr.swap(pivot_index as usize, high as usize);
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize); // i+1 is in bracket
    i + 1
}
