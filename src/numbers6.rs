pub fn run() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("numbers6")
}

// v = 1024   // decimal  
// + 255    // hex (0xff)
//  + 63     // octal (0o77)
//  + 255    // binary (0b1111_1111)
