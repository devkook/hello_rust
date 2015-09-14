#![allow(unused_imports)]

use hello_world::add;

#[test]
fn is_world_sane() {
  assert_eq!(2 + 2, 4);
}

#[test]
fn test_name() {
  assert_eq!(add(1, 2), 3);  
}
