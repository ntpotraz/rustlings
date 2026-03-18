use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    let n: u32 = input.trim().parse().expect("Enter a number");

    let fib = fibonacci(n);

    println!("{n}th Fibonacci number: {fib}")
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    }
    else {
        let mut prev = 1;
        let mut curr = 1;
        for _ in 1..n - 2 {
            let temp = prev;
            prev = curr;
            curr += temp;
        }

        curr + prev
    }
}
