/*
primitive types--

unassigned have no negative values
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128(numbers of bits taken in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, Which means that it must know the types of all the 
// variables at compile time, however, the compiler can usually infer what type we want to use 
// based on the value and how we use it.


pub fn types(){
    // Beginning of Types.rs
    println!("=========================Types.rs starts here========================\n");

    
    let x = 1;     // Default is i32

    let y = 2.4;   //default is f64

    let z: i64 = 4343254543234322;

    println!("Max i32 {}", std::i32::MAX);    // 2147483647
    println!("Max i64 {}", std::i64::MAX);    // 9223372036854775807

    // Boolean 
    let is_active:bool = true;

    // Get boolean from expression
    let is_greater: bool = 10<5;

    let a1 = 'k';         // character literal can have only one character or unicode value
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1,face));    // (1, 2.4, 4343254543234322, true, false)

    // End of print.rs
    println!("=========================T.Types starts here========================\n");

} 