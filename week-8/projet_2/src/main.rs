fn main() {
    // Vector of tuples: (name, years of experience)
    let candidates = vec![
        ("Mary", 3),
        ("John", 7),
        ("Ade", 5),
        ("Grace", 10),
        ("David", 4),
    ];

    // Assume the first candidate has the highest experience
    let mut highest = candidates[0];

    // Loop through the candidates
    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = *candidate;
        }
    }

    // Display the candidate with the highest experience
    println!("EY Interview Result");
    println!("Name: {}", highest.0);
    println!("Years of programming experience: {}", highest.1);
}
