// Fill the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..__), Range{ start: 1, end:: 5 });
//     assert_eq!((1..__), RangeInclusive::new(1, 5));

//     println!("numbers10")

// }

use std::ops::{Range, RangeInclusive};
pub fn run() {
    assert_eq!((1..5), Range{ start: 1, end: 5 }); // this is short ahnd and long hand/more verbose way of writing it. range is 1-5 where 5 is excluded.
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // RangeInclusive means that 5 would be included. Shorthand syntax is((1..=5))

    println!("numbers10")

}