/*
 * Default dangling pointer example
 */
#[allow(dead_code)]
pub fn dangling_pointer() {
    let _my_int_ptr: &i32;
    {
        let my_int: i32 = 5;
        _my_int_ptr = &my_int;
    }

    // This causes a compile error - 'borrowed value does not live long enough'
    // dbg!(*my_int_ptr);
}

/*
 * Borrowing a reference to a temporary object!
 */
#[allow(dead_code)]
pub fn borrowing_view() {
    let my_string: String = "abcdefgh".to_string();

    // Unable to get view &str to temporary string (my_string+z)
    // let my_string_view: &str = (my_string+"z").as_str();

    // Maybe like this
    let temp_string = my_string + "z";
    let my_string_view: &str = &temp_string;

    dbg!(my_string_view);
}

/*
 * Try putting a dangling pointer in a container, won't work
 */
#[allow(dead_code)]
pub fn dangling_pointer_in_container() {
    let mut my_vector: Vec<&str> = Vec::new();
    {
        let my_string = "hello world".to_string();
        my_vector.push(&my_string);
    }

    // Dropping my_string reference,unable to print use-after-free
    // dbg!(my_vector);
}

/*
 * invalid push back function
 */
#[allow(dead_code)]
#[allow(clippy::ptr_arg)]
fn invalid_push_back(_v: &mut Vec<&str>, _s: &str) {
    // Invalid push operation! s must outlive v
    // v.push(s);
}

/*
 * valid push back function - 'a is a lifetime annotation (we guarantee that v and s have the same lifetime)
 */
#[allow(dead_code)]
fn valid_push_back<'a>(v: &mut Vec<&'a str>, s: &'a str) {
    // Invalid push operation! s must outlive v
    v.push(s);
}

/*
 * Try putting a dangling pointer in a container (from a function)
 */
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn pushback_in_function() {
    let mut my_vector: Vec<&str> = Vec::new();
    {
        let my_string = "hello world".to_string();

        // This is an invalid function - error in the function signature
        // invalid_push_back(&mut my_vector, &my_string);

        // This function is valid but we can't call it because our string doesn't live long enough
        // valid_push_back(&mut my_vector, &my_string);
    }
    dbg!(my_vector);
}

/*
 * It is forbidden to make more than one mutable reference to the same object.
 */
pub fn mutable_aliasing() {
    let mut my_int = 5;
    let ref1 = &mut my_int;
    // let ref2 = &mut my_int;
    *ref1 += 1;
    // *ref2 +=1;

    // assert_eq!(my_int, 7);
}

/*
 * Cannot borrow something as immutable when its also borrowed as mutable.
 */
#[allow(unused_variables)]
pub fn mutable_aliasing_container() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let first = &mut char_array[0];

    // Can't do this! now char_arra[X] has one mutable and immutable ref.
    // let second = &char_array[1];

    // *first = *second;
    //assert_eq!(char_array[0], 'b');

    // How to fix this?
    // 1) Brute force indexing
    char_array[0] = char_array[1];
    assert_eq!(char_array[0], 'b');
}

/*
 * 2) Mutable aliasing using split_at_mut a.k.a. slices
 */
pub fn mutable_aliasing_slicing() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let (first_slice, second_slice) = char_array.split_at_mut(1);
    let first_element = &mut first_slice[0];
    let second_element = &second_slice[0];
    *first_element = *second_element;
    assert_eq!(char_array[0], 'b');
}

/*
 * 3) Mutable aliasing using iter_mut a.k.a. Iterators
 */
pub fn mutable_aliasing_iterator() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let mut it = char_array.iter_mut();

    let first = it.next();
    let second = it.next();

    // in other words if element first and second match the type Some(first_el),
    // Some(second_el)
    if let (Some(first_el), Some(second_el)) = (first, second) {
        *first_el = *second_el;
    }

    assert_eq!(char_array[0], 'b');
}

/*
 * 4) Mutable aliasing using cell
 * - Cell / Refcell implement interior mutability
 * - a.k.a. using Cell and Refcell we can mutate objects eventhough we only have shared shared pointers to them
 */
pub fn mutable_aliasing_cell() {
    // TODO:
}

/*
 * XX) Mutable aliasing using unsafe code
 */
pub fn mutable_aliasing_unsafe() {
    // XX) UNSAFE code
    // - Sometimes used to call into C functions (C can do anything >:D)
    let mut char_array: [char; 2] = ['a', 'b'];
    let first_element: *mut char = &mut char_array[0];
    let second_element: *const char = &char_array[1];

    // spooky...
    unsafe {
        *first_element = *second_element;
    }
    assert_eq!(char_array[0], 'b');
}

#[allow(dead_code)]
fn push_int_twice(v: &mut Vec<i32>, n: &i32) {
    v.push(*n);
    v.push(*n);
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn reallocating_invalidates_reference() {
    let mut my_vector = [0];
    let my_int_reference = &my_vector[0];

    // Can't call, double mutable borrow
    // push_int_twice(&mut my_vector, my_int_reference);
}
