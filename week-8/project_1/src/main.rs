use std::io;

fn error_a(){
    println!("Invalid input");
    println!("Input from the given options");
}

fn error_b(){
    println!("Invalid input");
    println!("Input a number from 0 to 5");
}

fn error_c() {
    println!("Invalid input");
    println!("Input a number from 1 to 5");
}
fn main() {

    let public_service = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    println!("GOOD DAY MR. CHUDI!");
    println!("WELCOME TO THE APS LEVEL CHECKER");
    println!("To begin input enter your field of choice: ");

    println!("Enter O if you fall into the Office Admin field");
    println!("Enter A if you fall into the Academic field");
    println!("Enter L if you fall into the Lawyer field");
    println!("Enter T if you fall into the Teacher field");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let category = input1.trim().to_uppercase();

    if category == "O" {
        println!("\nYou are in the Office Admin field");
        println!("Great, let's determine your level in the public service");
        let mut level1 = String::new();

        println!("\nEnter 0 if you are an Intern");
        println!("Enter 1 if you are an Administrator");
        println!("Enter 2 if you are a Senior Administrator");
        println!("Enter 3 if you are an Office Manager");
        println!("Enter 4 if you are a Director");
        println!("Enter 5 if you are a CEO");

        io::stdin().read_line(&mut level1).unwrap();
        let index1:usize = level1.trim().parse().unwrap();
        if index1>5 {
            error_b();
        }

        println!("\nIn the Public Service, you are at the level of {}", public_service[index1]);
    }   
     else if category == "A" {
          println!("\nYou are in the academic field");
          println!("Great, let's determine your level in the public service");
          let mut level2 = String::new();

          println!("Enter 1 if you are a Research Assistant");
          println!("Enter 2 if you are a phD Candidate");
          println!("Enter 3 if you are a Post-Doc Researcher");
          println!("Enter 4 if you are a Senior Lecturer");
          println!("Enter 5 if you are a Dean");
          
          io::stdin().read_line(&mut level2).unwrap();
          let index2:usize = level2.trim().parse().unwrap();
          if index2<1 || index2>5 {
            error_c();
          }
          
          println!("\nIn the Public Service, you are at the level of {}", public_service[index2]);
     }

     else if category =="L" {
        println!("\nYou are in the lawyer field");
        println!("Great, let's determine your level in the public service");
        let mut level3 = String::new();

        println!("\nEnter 0 if you are a Paralegal");
        println!("Enter 1 if you are a Junior Associate");
        println!("Enter 2 if you are an Associate");
        println!("Enter 3 if you are a Senior Associate 1-2");
        println!("Enter 4 if you are a Senior Associate 3-4");
        println!("Enter 5 i you are a Partner");

        io::stdin().read_line(&mut level3).unwrap();
        let index3:usize = level3.trim().parse().unwrap();
        if index3>5 {
            error_b();
        }

        println!("\nIn the Public Service, you are at the level of {}", public_service[index3]);

     }

     else if category == "T" {
        println!("\nYou are a Teacher");
        println!("Great, let's determine your level in the public service");
        let mut level4 = String::new();

        println!("\nEnter 0 if you are a Placement");
        println!("Enter 1 if you are a Classroom Teacher");
        println!("Enter 2 if you are a Senior Teacher");
        println!("Enter 3 if you are a Leading Teacher");
        println!("Enter 4 if you are a Deputy Principal");
        println!("Enter 5 if you are a Principal");

        io::stdin().read_line(&mut level4).unwrap();
        let index4:usize = level4.trim().parse().unwrap();
        if index4>5 {
            error_b();
        }

        println!("\nIn the Public Service, you are at the level of {}", public_service[index4]);
     }

     else {
        error_a();
     }
}