#![allow(dead_code)]

pub mod arrays;

// Not used below
fn learning_by_doing() {
    println!("I'd like to learn the Rust Language.")
}

#[cfg(test)]
#[test]
#[ignore = "example not used"]
fn it_works() {
    learning_by_doing();
    assert!(learning_by_doing() == ());
}
