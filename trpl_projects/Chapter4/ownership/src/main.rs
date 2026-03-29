fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("Initial v: {:?}", v);

    let num: &mut i32 = &mut v[2];
    println!("Third element prior is: {}", *num);
    *num += 1;
    println!("Third element after is: {}", *num);
    println!("v: {:?}", v);
}



