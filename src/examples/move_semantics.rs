use std::mem;
use std::fs::File; 

/*
 * Moving a non-POD destroys the object.
 * This is the default behaviour on assignment.
 */
#[allow(clippy::vec_init_then_push)]
pub fn moving_a_string() {
    let s1 = "abcdef".to_string();
    let s2 = s1; // a move happens here, s1 is gone

    // Unable to do anything with s1 e.g.
    // let b = s1.clone();

    let mut v: Vec<String> = Vec::new();
    v.push(s2); // Another move happens

    // Unable to do anything with s2
    // let b = s2.clone();

    assert_eq!(v[0], "abcdef".to_string());
}

#[allow(clippy::vec_init_then_push)]
pub fn copying_a_string() {
    let s1 = "abcdef".to_string();
    let s2 = s1.clone(); // now it copies

    // Can do stuff still with s1 e.g.
    let _b = s1.clone();

    let mut v: Vec<String> = Vec::new();
    v.push(s2); // Another move happens

    // Unable to do anything with s2
    // let b = s2.clone();

    assert_eq!(v[0], "abcdef".to_string());
}

/*
 * Unable to use the view to the string after the original string is moved away.
 */
#[allow(unused_variables)]
pub fn moved_string_view() {
    let s1 = "abcde".to_string();
    let my_view = s1.as_str();
    let s2 = s1; // here it's fine

    // Here it breaks!
    // Unable to use view to the value after its moved
    // dbg!(my_view);
}

/*
 * Move string located in a container. Should work in c++ - although not advised.
 */
#[allow(unused_variables)]
#[allow(clippy::vec_init_then_push)]
pub fn moved_string_from_container() {
    let s1 = "abcde".to_string();
    let s2 = s1;

    let mut v: Vec<String> = Vec::new();
    v.push(s2);

    //So far so good

    // Here it breaks!
    // let s3 = v[0];

    // Rust would want to evaporate v[0] - vanish
    // - How would that work ?
    // - Does it evaporate the entire vector?
    // - How does it record which elements are 'evaporated'?
    // - It would essentially have to pop the 0th element from vector.
    // - It's not really clear what it should do.
    //
    // Rust suggests that v[0]'s type should implement Copy. It doesn't because it's not a POD type
}

#[allow(unused_variables)]
#[allow(clippy::ptr_arg)]
fn move_ref(s1: &mut String) {
    // Cannot do this because we invalidate the mutable reference
    // We don't know where does s1 come from, where does it live?

    // let s2 = *s1;
    // dbg!(s2);
}

/**
 * Move a reference in a function.
 */
pub fn moving_through_reference() {
    let mut s1 = "abcde".to_string();
    move_ref(&mut s1);
    dbg!(s1);
}

/*
 * Move around mutable references using std::swap
 */
pub fn memswap_move() {
    let mut s1 = "foo".to_string();

    // make a memswap lambda
    let memswap_string = |s1: &mut String| {
        let mut s2 = "".to_string();

        // We may not rip s1 out of existance, instead we can swap it with an empty value
        mem::swap(s1, &mut s2);
        dbg!(s2);
    };

    memswap_string(&mut s1);
    assert_eq!("".to_string(), s1);
}

/*
 * Move values out of variables by wrapping them in Option<T>.
 */
pub fn option_move() {
    let mut s1: Option<String> = Some("foo".to_string());

    let option_move_lambda = |s1: &mut Option<String>| {
        let s2 = s1.take().unwrap();
        dbg!(s2);
    };

    option_move_lambda(&mut s1);
    dbg!(s1);
}

/*
 * (re)Move objects outside of vector using Vec::remove
 */
pub fn container_remove() {
    let mut v = vec!["foo".to_string(), "bar".to_string()];

    let vector_rm = |v: &mut Vec<String>| {
        // "pop" 0-th element out of vector v
        let s2 = v.remove(0);
        dbg!(s2);
    };

    vector_rm(&mut v);
    dbg!(v);
}

/*
 * Move objects nowhere.
 */
pub fn move_to_nowhere() {
    let file = File::open("/dev/null");
    drop(file);
}

