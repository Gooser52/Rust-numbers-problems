
// // Modify `assert_eq!` to make it work
// fn main() {
//     let x = 5;
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }


// Modify `assert_eq!` to make it work
pub fn run() {
    let x: u32 = 5; // default type is i32, so we need to change to u32
    assert_eq!("u32".to_string(), type_of(&x));

    println!("numbers3");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}