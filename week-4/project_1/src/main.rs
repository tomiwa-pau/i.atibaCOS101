//Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter a value of a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Enter a value of b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter a value of c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    let d:f32 = b * b - 4.0 * a * c;
    if d > 0.0 {
    println!("Two distinct roots");
} else if d == 0.0 {
    println!("Exactly one real root");
} else {
    println!("No real roots");
}


}