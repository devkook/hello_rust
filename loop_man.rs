fn main() {
  let mut y = 0;

  for x in 0..1_000_000_000 {
        y = y + x;
        y = y - x;
  }
  println!("{}",y);
}
