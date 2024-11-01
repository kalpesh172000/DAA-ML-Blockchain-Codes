use std::io::Write;
fn main()
{
    let mut inputstream = String::new();
    print!("Enter how many fibonacci numbers you want to be printed: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut inputstream).expect("Failed to read input"); 
    let n:u16 = inputstream.trim().parse().expect("Invalid input");
    let mut a = 0;
    let mut b = 1;
    println!("1. {}",a);
    println!("2. {}",b);
    for i in 2..n{
        let c = a+b;
        println!("{}. {}",i+1,c);
        a = b;
        b = c;
    }
}