use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a temperature in Fahrenheit");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading line");

    let fahrenheit: f64 = input.trim().parse().expect("Enter a number");
    let celcius = f_to_c(fahrenheit);

    println!("{fahrenheit}F in Celcius is {celcius}C");
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}
