# Rust Tutorial

[A Firehose of Rust](https://www.youtube.com/watch?v=FSyfZVuD32Y&ab_channel=JackO%27Connor)
[Slides](https://jacko.io/firehose_of_rust/)

## References and Mutable Aliasing
  - shared references,  &T (like const pointers in C++)
  - mutable references, &mut T, like non-const pointers in c++
  - References don't keep things alive!! There is no garbage collection

### BIG#1 - there is NO use-after-free. 
 - References are always valid
 - No dangling (NULL) pointers
 - Rust compiler proves (statically) at COMPILE TIME that we haven't kept references or pointers longer than we should

### BIG#2 - MUTABLE reference(aliasing) are UNIQUE!
 - no aliasing on mutable references
 - if i want to take const reference to same T -> FINE
 - if i want to make more mut. references to same T -> WRONG

### BIG#3 - by-value operations on non-```Copy``` types are MOVES (destructive operations), EVERYTHING IS MOVABLE!
 - "plain old data" (in C++ is trivially copyable) types (i32, &T) are ```Copy``` (trait)
 - by-value operations on POD types are bitwise copies
 - types like ```Vec<i32``` or ```&mut T``` are non-```Copy```
 - *NOTE* - The meaning of destructive - the destructor of the source(from move) is NEVER called (in C++ destructor runs after std::move(..))
 - *EVERYTHING IS MOVABLE!*
 - moves are always bitwise
 - "In Rust, a copy is a move that doesn't destroy its source"
 
