
use std::io::Write;

fn main() {
    let commissioner = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimah Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministry = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zone = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file =
        std::fs::File::create("The EFCC Merge.csv")
            .expect("create failed");

    writeln!(file, "Commissioner,Ministry,Zone").unwrap();

    for i in 0..commissioner.len() {
        writeln!(
            file,
            "{},{},{}",
            commissioner[i],
            ministry[i],
            zone[i]
        )
        .unwrap();
    }

    println!("EFCC data merged successfully!");
    println!("File created: The EFCC Merge.csv");
}
