use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut inputstream = String::new();
    print!("Enter how many fibonacci numbers you want to be printed: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputstream).expect("failed to read input");
    let n: i32 = inputstream.trim().parse().expect("Invalid input");

    let mut memo: HashMap<i32, i32> = HashMap::new();
    memo.insert(0,0);
    memo.insert(1,1);

    fibo(n, &mut memo);
    for i in 0..n {
        println!("{}. {}", i+1, memo.get(&i).unwrap());
    }
}

fn fibo(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n <= 1 {
        return n;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = fibo(n - 1, memo) + fibo(n - 2, memo);
    memo.insert(n, result);
    result
}
