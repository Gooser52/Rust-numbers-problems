
// // Fill the blanks and fix the errors
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == __);

//     // Integer subtraction
//     assert!(1i32 - 2 == __);
//     assert!(1u8 - 2 == -1); 
    
//     assert!(3 * 50 == __);

//     assert!(9.6 / 3.2 == 3.0); // error ! make it work

//     assert!(24 % 5 == __);
//     // Short-circuiting boolean logic
//     assert!(true && false == __);
//     assert!(true || false == __);
//     assert!(!true == __);

//     // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }


// Fill the blanks and fix the errors
pub fn run() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1); 
    
    assert!(3 * 50 == 150); // default i32

    assert!(9.6 as f32/ 3.2 as f32== 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4); // modulus (%) gives the remainder not the full division i.e 4 not 4.8 as 5 goes into 24 4 full times
    // Short-circuiting boolean logic
    assert!(true && false == false); // in AND boolean both inputs must be true to output true
    assert!(true || false == true); // in an OR boolean only one input has to be true to output true
    assert!(!true == false); // not true is equal to false

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0 AND 0 is 0, 0 AND 1 is 0, 1 AND 1 is 1 so this equals 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); // 0111 because 1 OR 0 is 1, 1 OR 1 is 1, 0 OR 0 is 0
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0110 , in XOR the inputs must be different for it to output 1
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}