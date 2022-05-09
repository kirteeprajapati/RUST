// Structs- are used to create custom data types

// Traditional Struct
struct Color{
        red: u8,
        green: u8,
        blue: u8,
}

// Tuple Struct
struct Color_tup(u8, u8, u8);

struct Person{
    first_name: String,
    last_name: String
}

impl Person{
    // Construct Person
    fn new(first: &str, last:&str)->Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
}

pub fun structs(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };
        c.red = 200;
        
    println!("Color: {} {} {}", c.red, c.green, c.blue);
        
    let mut c_tup = Color_tup(255, 0, 0);
    println!("Color: {} {} {}", c_tup.0, c_tup.1, c_tup.2);
    
    c.0 = 200;
    println!("Color: {} {} {}", c_tup.0, c_tup.1, c_tup.2);
}
