# Rust Tutorial

## References and Mutable Aliasing

  - shared references,  &T (like const pointers in C++)
  - mutable references, &mut T, like non-const pointers in c++
  - References don't keep things alive!! There is no garbage collection

### BIG#1 - there is NO use-after-free. 
 - References are always valid
 - No dangling (NULL) pointers
 - Rust compiler proves (statically) at COMPILE TIME that we haven't kept references or pointers longer than we should

### BIG#2 - MUTABLE reference are UNIQUE!
 - no aliasing on mutable references
 - if i want to take const reference to same T -> FINE
 - if i want to make more mut. references to same T -> WRONG

