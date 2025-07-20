// fn main() {
//     let v1 = 251_u8 + 8;
//     let v2 = i8::checked_add(251, 8) .unwrap();
//     println! ({},{},v1,v2);
// }

pub fn run() {
    let v1 = 251_u16 + 8; //u8 can only hold a value of 255, so 259 is not possible, change to u16
    let v2 = i16::checked_add(251, 8) .unwrap(); //i8 causes an overflow because it cannot hold 259, so changed to i16 which can handle it
    println! ("{},{}",v1,v2);
}