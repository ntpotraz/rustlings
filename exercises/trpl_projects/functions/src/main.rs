use std::io;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();

    println!("Please enter a number");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    let input: i32 = input.trim().parse().expect("Enter a number");

    another_function(input);
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}
