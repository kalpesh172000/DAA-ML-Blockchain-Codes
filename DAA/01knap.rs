fn knapsack(capacity: i32, weights: &[i32], profits: &[i32], n: usize) -> i32 {
    if n == 0 || capacity == 0 {
        return 0;
    }

    if weights[n - 1] > capacity {
        //we don't use & here cause its already a reference 
        return knapsack(capacity, weights, profits, n - 1);
    }

    // (1) nth item included
    // (2) not included
    return std::cmp::max (
        knapsack (capacity,&weights, &profits, n-1 ) ,
        profits[n-1] + knapsack(capacity-weights[n-1],weights,profits,n-1)
    )
}

fn main() {
    let weights = [10, 20, 30];
    let profits = [60, 100, 120];
    let capacity = 50;
    let n = weights.len();
    println!("Maximum profit : {} ",knapsack(capacity,&weights,&profits,n))
}
