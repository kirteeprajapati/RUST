// Variables hold primitive data or references to data
// Variable are immutable by default
// Rust is a block-scoped language

pub fn var(){
    let name = "Kirtee";
    let mut age = 19;
    println!("My name is {}, M {} from {}", name, age, "India");
    age = 59;
    println!("My name is {}, M {} from {}", name, age, "India");

    // Define Constant 
    const ID: i32 = 24227501;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Kirtee", 19);
    println!("{} is {}", my_name, my_age);
}