
use std::io::Write;

fn main() {
    let student_name = vec![
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_number = vec![
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let department = vec![
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let level = vec![
        "300",
        "100",
        "200",
        "200",
        "100",
    ];

    // Creating the file
    let mut file =
        std::fs::File::create("PAU-SMIS.csv")
            .expect("create failed");

    // Writing the heading
    writeln!(
        file,
        "Student Name,Matric Number,Department,Level"
    ).unwrap();

    // To Display student details
    println!("PAU STUDENT MANAGEMENT INFORMATION SYSTEM");
    println!("=====================================");

    // Read and save student information
    for i in 0..student_name.len() {
        println!(
            "{}, {}, {}, {}",
            student_name[i],
            matric_number[i],
            department[i],
            level[i]
        );

        writeln!(
            file,
            "{},{},{},{}",
            student_name[i],
            matric_number[i],
            department[i],
            level[i]
        ).unwrap();
    }

    println!("\nStudent records saved successfully!");
    println!("File created: PAU-SMIS.csv");
}

