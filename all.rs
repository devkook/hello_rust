pub mod hello_world;
mod hello_world_test;

use hello_world::add;

fn main() {
  println!("Hello, 2 + 2 is {}", add(2, 2));
}
