// Tuples group together values of different types
// Max 12 elements

pub fn tup(){
    // Beginning of Tuples.rs
    println!("=========================Tuples.rs starts here========================\n");

    let person: (&str, &str, i32) = ("Kirtee", "IIITA India", 19);
    println!("{} is pursuing her BTech from {} and is {} years old", person.0, person.1, person.2);

    // End of Tuples.rs
    println!("=========================Tuples.rs starts here========================\n");

}
