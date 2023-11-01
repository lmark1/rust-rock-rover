// Dangling pointer example
#[allow(dead_code)]
fn dangling_pointer() {
    let _my_int_ptr: &i32;
    {
        let my_int: i32 = 5;
        _my_int_ptr = &my_int;
    }
    // This causes a compile error - 'borrowed value does not live long enough'
    // dbg!(*my_int_ptr);
}

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
