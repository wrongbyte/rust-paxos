pub mod actors;
pub mod domain;
pub mod repository;

/// General rules
/// Only a value that has been proposed may be chosen.
/// A process never learns that a value has been chosen unless it actually has been.

fn main() {
    println!("Hello, world!");
}
