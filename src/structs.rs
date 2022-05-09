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

impl Person{                            //impl = impliment
    // Construct Person
    fn new(first: &str, last:&str)->Person{
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    // Get full name
    fn full_name(&self)->String{
        format!("{} {}", self.first_name, self.last_name)
    }
        
    // Set last name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }
        
    // Name to tuple
    fn to_tuple(self)->(String, String){
        (self.first_name, self.last_name)
    }
}

pub fun structs(){
        
    // Beginning of Structs
    println!("=========================Structs.rs starts here========================\n");

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
        
    let mut p = Person::new("Kirtee", "Prajapati");
    println!("Person {} {}", p.first_name, p.last_name);
        
    println!("Person {}", p.full_name());
        
    p.set_last_name("Kumari");
    println!("Person {}", p.full_name());    
        
    println!("Person Tuple {:?}", p.to_tuple());
    
    // End of Structs
    println!("=========================Structs.rs Ends here========================\n");

}
