
use std::io;

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(number) if number >= 0.0 => return number,
            _ => println!("Please enter a valid non-negative number."),
        }
    }
}

fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter the first base:");
    let base2 = read_number("Enter the second base:");

    height / 2.0 * (base1 + base2)
}

fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter the first diagonal:");
    let diagonal2 = read_number("Enter the second diagonal:");

    0.5 * diagonal1 * diagonal2
}

fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");

    base * altitude
}

fn cube_surface_area() -> f64 {
    let side = read_number("Enter the length of one side:");

    6.0 * side * side
}

fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");

    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("=== SHAPE CALCULATOR ===");
    println!("1. Area of a Trapezium");
    println!("2. Area of a Rhombus");
    println!("3. Area of a Parallelogram");
    println!("4. Surface Area of a Cube");
    println!("5. Volume of a Cylinder");

    let choice = read_number("Choose a shape (1-5):");

    let result = match choice as u32 {
        1 if choice == 1.0 => trapezium_area(),
        2 if choice == 2.0 => rhombus_area(),
        3 if choice == 3.0 => parallelogram_area(),
        4 if choice == 4.0 => cube_surface_area(),
        5 if choice == 5.0 => cylinder_volume(),
        _ => {
            println!("Invalid choice. Please select a number from 1 to 5.");
            return;
        }
    };

    println!("The result is: {:.2}", result);
}

