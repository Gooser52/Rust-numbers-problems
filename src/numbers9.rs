// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -3);

//     for c in 'a'..='z' {
//         println!("{}",c);
//     }
// }
// 🌟🌟 Two goals: 1. Modify assert! to make it work 2. Make println! output list of numbers between 97 and 122

pub fn run() {
    let mut sum: i32 = 0; // initla value is 0

    for i in -3..2 { // this is a for loop so i takes values -3, -2, -1, 0, 1
        sum += i // sum becomes: -3, -5, -6, -6, -5
    }

    assert!(sum == -5); // this is -5 after

    for c in 'a'..='z' { // this prints letters a - z
        println!("{}",c as u8); // instead of outputting c as a letter, we want to output it as a u8 integer (97-122 are represented by a-z in ASCII table)
    }
}