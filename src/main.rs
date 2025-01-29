use std::io;

fn main() {
    println!("Welcome to the nth Fibonacci number finder app!");

    loop {
        println!("Enter the position of the Fibonacci number in the sequence (index starts at 0):");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: i128 = match n.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input! Try again.");
                continue;
            },
        };
        
        let value: i128 = { fib_num(n) };

        println!("The Fibonacci number in position {} is: {}", n, value);
        break;
    }
}

fn fib_num(n: i128) -> i128 {
    let mut first: i128 = 0;
    let mut second: i128 = 1;
    let mut counter: i128 = 0;

    let ans: i128 = if n == 0 { first } else if n == 1 { second }
        else {
            let mut val: i128 = 0;
            while counter < n {
                let this: i128 = first + second;
                val = this;
                first = second;
                second = this;
                counter += 1;
            }
        val
        };
    ans
}
