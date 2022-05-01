pub fn run(){
    // print in console
    println!("Hello World from console via print.rs file");

    // must use placeholder
    println!("Number to be printed: {}", 1); 
    
    // Positional arguments
    println!("Multiple placeholder so we can display {}, {} and {} too in the same print with index to {2}", "elephant", "giraffe", "monkey");

    // Named arguments 
    println!("Hey {name}, How's your {activity} going", name= "Kirtee", activity="Intern Prep");

    //Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}.", 10, 10 , 10);                         //Binary: 1010, Hex: a, Octal: 12.

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));    // Printing tuple 

    // Basic Math
    println!("10 +10 = {}", 10+10);
}