// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn strings(){
    let hello = "Hello Rust!";   //Immutable fixed-length 
    let mut growable_str = String::from("Hello this sting is growable and heap-allocated data type");
    println!("Immutable string: {}, Mutable string: {}", hello, growable_str);

    // Get length of sting
    println!("Length of the mutable string: {}", growable_str.len());
    println!("Length of the Immutable string: {}", hello.len());

    growable_str.push('w');         //push a character
    growable_str.push_str("orld");  //push a string
    // hello.push('H');                 //method not found in `&str`
    println!("Length of the mutable string: {}", growable_str.len());
    println!("Length of the Immutable string: {}", hello.len());

    // Capacity in bites
    println!("Capacity: {}", growable_str.capacity());

    // Checks if empty
    println!("Is Empty: {}", growable_str.is_empty());

    // Check if contains
    println!("Contains 'this': {}", growable_str.contains("this"));

    // Replace 
    println!("Replace 'this' with 'there' {}", growable_str.replace("this", "there! this"));

    // Loop through string by whitespace
    for word in growable_str.split_whitespace(){
        println!("{}", word);                        // separates each word in different line
    }

    // Create string with certain capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{} {}", growable_str, s);


}