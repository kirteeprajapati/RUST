// Structs- are used to create custom data types

// Traditional Struct
struct Color{
        red: u8,
        green: u8,
        blue: u8,
}
pub fun structs(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Color: {} {} {}", c.red, c.green, c.blue);
}
