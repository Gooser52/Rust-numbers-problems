// Integer
// 🌟
// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

// Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;

//     y = x;
    
//     let z = 10; // Type of z ? 

//     println!("Success!");
// }

// Remove something to make it work
pub fn run() {
    let x: i32 = 5;
    let mut _y = 5; //remove u32 because different integer types cannot be made equal. i32 is default so implied

    // You can modify y now
    _y = x;

    let _z = 10; // Type of z ? i32 because it is default. (added _ to z because unused variable)

    println!("Success!");
}
