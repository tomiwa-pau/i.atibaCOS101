//Rust program to create the incentive calculator

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter employee experience (experienced or not experienced):");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let experience:String = input1.trim().parse().expect("Not a valid input");

    println!("Enter employee age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    if experience == "experienced" && age >= 40 {
        let _incentive:f32  = 1_560_000.00;
        println!("Your incentive is {}", _incentive);
    }
    else if experience == "experienced" && age >= 30 && age <= 39 {
         let _incentive:f32 = 1_480_000.00;
         println!("Your incentive is {}", _incentive);
    }
     else if experience == "experienced" && age <= 29 {
        let _incentive:f32 = 1_300_000.00;
        println!("Your incentive is {}", _incentive);
     }    
      else if experience == "not experienced" {
        let _incentive:f32 = 100_000.00;
         println!("Your incentive is {}", _incentive);
 }
         

}