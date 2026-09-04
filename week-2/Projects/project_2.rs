 fn main () {
  // I'm using the short form of their names
  let toshsum:f64 = 2.0 * 450_000.00;
  let macsum:f64 = 1.0 * 1_500_000.00;
  let hpsum:f64 = 3.0 * 750_000.00;
  let dellsum:f64 = 3.0 * 2_850_000.00;
  let acersum:f64 = 1.0 * 250_000.00;
  let totalsum = toshsum + macsum + hpsum + dellsum + acersum;
  
  let totalqty:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

  let avg = totalsum/totalqty;
  println!("The total average is {} and the total sum is {}",avg, totalsum );
}