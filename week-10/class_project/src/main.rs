struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn purchase_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650_000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755_000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550_000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850_000,
        },
    ];

    let quantity = 3;
    let mut total_cost = 0;
    println!("GOOD DAY MR CHUDI!");
    println!("LAPTOP PURCHASE SUMMARY");
    
    for laptop in &laptops {
        let cost = laptop.purchase_cost(quantity);
        total_cost += cost;

        println!(
            "Brand: {} | Quantity: {} | Total: N{}",
            laptop.brand, quantity, cost
        );
    }

    println!("------------------------------");
    println!("Grand Total: N{}", total_cost);
}
