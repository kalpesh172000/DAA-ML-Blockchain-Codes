use std::io::Write;
fn main()
{
    let mut inputstream = String::new();
    print!("Enter how many fibonacci numbers you want to be printed: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputstream).expect("Failed to read input"); 
    let n:i16 = inputstream.trim().parse().expect("Invalid input");
    println!("1. {}",&0);
    println!("2. {}",&1);
    for i in 2..n{
        println!("{}. {}",i+1,fibonacci(i-1));
    }
}

fn fibonacci(n:i16) -> i16
{
    if n<=1
    {
        return n;
    }
    return fibonacci(n-1) + fibonacci (n-2);
}