
use postgres::{Client, NoTls, SimpleQueryMessage};
use std::error::Error;
use std::io::{self, Write};

fn display_table(
    client: &mut Client,
    table: &str,
) -> Result<(), Box<dyn Error>> {

    println!("\n========================================");
    println!("         {} TABLE", table.to_uppercase());
    println!("========================================");

    // Display the table columns
    let columns = client.query(
        "SELECT column_name
         FROM information_schema.columns
         WHERE table_schema = 'public'
         AND table_name = $1
         ORDER BY ordinal_position",
        &[&table],
    )?;

    for column in &columns {
        let name: String = column.get(0);
        print!("{:<20}", name);
    }

    println!("\n");

    // Retrieve and display the table records
    let sql = format!(
        "SELECT * FROM public.{} ORDER BY 1",
        table
    );

    let messages = client.simple_query(&sql)?;

    for message in messages {
        if let SimpleQueryMessage::Row(row) = message {
            for i in 0..row.len() {
                let value = row.get(i).unwrap_or("NULL");
                print!("{:<20}", value);
            }
            println!();
        }
    }

    println!("\nTable displayed successfully!");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

    // Connect to PostgreSQL
    let mut client = Client::connect(
        "host=localhost user=postgres password=cos101 dbname=globacom_dbase port=5432",
        NoTls,
    )?;

    println!("========================================");
    println!("       GLOBACOM DATABASE SYSTEM");
    println!("========================================");

    println!("\nSelect your role:");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    print!("\nEnter your choice: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {

        "1" => {
            println!("\nWelcome Administrator!");
            println!("You can view the entire database.");

            display_table(&mut client, "project")?;
            display_table(&mut client, "staff")?;
            display_table(&mut client, "customer")?;
            display_table(&mut client, "dataplan")?;
        }

        "2" => {
            println!("\nWelcome Project Manager!");
            display_table(&mut client, "project")?;
        }

        "3" => {
            println!("\nWelcome Employee!");
            display_table(&mut client, "staff")?;
        }

        "4" => {
            println!("\nWelcome Customer!");
            display_table(&mut client, "customer")?;
        }

        "5" => {
            println!("\nWelcome Vendor!");
            display_table(&mut client, "dataplan")?;
        }

        _ => {
            println!("\nInvalid choice! Please select 1-5.");
        }
    }

    Ok(())
}

