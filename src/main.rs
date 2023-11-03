// Get all public function from examples/memory.rs into scope here
use crate::examples::memory;
use crate::examples::threading;
use crate::examples::move_semantics;

// Use this to read examples/mod.rs file
pub mod examples;

fn main() {
    println!("Hekkers");

    // BIG#1 - There is no use after free
    memory::dangling_pointer();
    memory::borrowing_view();
    memory::dangling_pointer_in_container();
    memory::pushback_in_function();

    // BIG#2 - Mutable aliasing is unique
    memory::mutable_aliasing();
    memory::mutable_aliasing_container();
    memory::mutable_aliasing_slicing();
    memory::mutable_aliasing_iterator();
    memory::mutable_aliasing_cell();
    memory::mutable_aliasing_unsafe();
    memory::reallocating_invalidates_reference();

    // BIG#2 - Aliasing while threading
    threading::magical_multithreading_aliasing();
    threading::tragical_multithreading_aliasing();
    threading::sync_shared_state_rwlock();
    threading::sync_shared_state_atomic();
    threading::sync_shared_state_mutex();

    // BIG#3 - by-value on nonCopy moves the object
   move_semantics::moving_a_string(); 
   move_semantics::moved_string_view();
   move_semantics::memswap_move();
   move_semantics::option_move();
   move_semantics::container_remove();
   move_semantics::move_to_nowhere();
}
