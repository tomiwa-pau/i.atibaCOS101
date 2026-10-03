use std::io;

fn main() {
    println!("===== FOOD MENU =====");
    println!("P - Poundo Yam / Edikangiko Soup - N3200");
    println!("F - Fried Rice & Chicken - N3000");
    println!("A - Amala & Ewedu Soup - N2500");
    println!("E - Eba & Egusi Soup - N2000");
    println!("W - White Rice & Stew - N2500");

    // Get food type
    println!("\nEnter food type:");
    let mut food = String::new();
    io::stdin().read_line(&mut food).unwrap();

    let food = food.trim().to_uppercase();

    // Get quantity
    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).unwrap();

    let quantity: i32 = quantity.trim().parse().unwrap();

    // Determine price
    let price: i32 = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    // Calculate total
    let total = price * quantity;

    println!("Total before discount: N{}", total);

    // Apply 5% discount if total is greater than N10,000
    if total > 10000 {
        let discount = total as f64 * 0.05;
        let final_total = total as f64 - discount;

        println!("Discount: N{}", discount);
        println!("Final amount: N{}", final_total);
    } else {
        println!("No discount.");
        println!("Final amount: N{}", total);
    }
}
