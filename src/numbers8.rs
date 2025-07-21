
// fn main() {
//     assert!(0.1+0.2==0.3);

//     println!("Success!");
// }

//make it work in 2 distinct ways

pub fn run() {
    assert!(0.1_f32+0.2_f32==0.3_f32);

    println!("numbers8");
}

// f64 is too precise for these values, because the true value would be a tiny fraction i.e 0.10000000004. Changed to f32 for these values.


// pub fn run() {
//     assert!(0.1 as f32+0.2 as f32==0.3 as f32);

//     println!("Success!");
// }