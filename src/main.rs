// Get all public function from examples/memory.rs into scope here
use crate::examples::memory;

// Use this to read examples/mod.rs file
pub mod examples;

fn main() {
    println!("Hekkers"); 
    memory::borrowing_view();
}
