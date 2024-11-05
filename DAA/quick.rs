//rand = "0.8.5"
use rand::Rng;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    
    // Create a sample array
    let size = 1000; // Change the size for analysis
    let mut arr_deterministic: Vec<i32> = (0..size).map(|_| rng.gen_range(0..10000)).collect();
    let mut arr_randomized: Vec<i32> = arr_deterministic.clone(); // Clone for randomized sort

    // Calculate array lengths in advance to avoid borrow conflicts
    let len_deterministic = arr_deterministic.len() as isize;
    let len_randomized = arr_randomized.len() as isize;

    // Analyze deterministic quicksort
    let start = Instant::now();
    quicksort_deterministic(&mut arr_deterministic, 0, len_deterministic - 1);
    let duration_deterministic = start.elapsed();
    
    // Analyze randomized quicksort
    let start = Instant::now();
    quicksort_randomized(&mut arr_randomized, 0, len_randomized - 1);
    let duration_randomized = start.elapsed();
    
    // Output the results
    println!("Deterministic QuickSort took: {:?}", duration_deterministic);
    println!("Randomized QuickSort took: {:?}", duration_randomized);
}

// Deterministic QuickSort
fn quicksort_deterministic(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition_deterministic(arr, low, high);
        quicksort_deterministic(arr, low, p - 1);
        quicksort_deterministic(arr, p + 1, high);
    }
}

fn partition_deterministic(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];//last element
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}

// Randomized QuickSort
fn quicksort_randomized(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        let p = partition_randomized(arr, low, high);
        quicksort_randomized(arr, low, p - 1);
        quicksort_randomized(arr, p + 1, high);
    }
}

fn partition_randomized(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot_index = low + rand::thread_rng().gen_range(0..(high - low + 1));
    arr.swap(pivot_index as usize, high as usize); // Move pivot to end
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap((i + 1) as usize, high as usize);
    i + 1
}
