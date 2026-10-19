use std::io::Write;

fn main() {

    let header = "Lager, Stout, Non-Alcoholic\n";
    let data1 = "33 Export, Legend, Maltina\n";
    let data2 = "Desperados, Turbo King, Amstel Malta\n";
    let data3 = "Goldberg, Williams, Malta Gold\n";
    let data4 = "Gulder,, Fayrouz\n";
    let data5 = "Heineken\n";
    let data6 = "Star";

   let mut file = std::fs::File::create("Nigerian Breweries.csv").expect("create failed");
   file.write_all(header.as_bytes()).expect("write failed");
   file.write_all(data1.as_bytes()).expect("write failed");
   file.write_all(data2.as_bytes()).expect("write failed");
   file.write_all(data3.as_bytes()).expect("write failed");
   file.write_all(data4.as_bytes()).expect("write failed");
   file.write_all(data5.as_bytes()).expect("write failed");
   file.write_all(data6.as_bytes()).expect("write failed");
   println!("\nData written to file." );
}