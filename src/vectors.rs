// Vectors are resizable arrays
use std::mem;
pub fn vectors(){
    // Beginning of Vector.rs
    println!("=========================Vectors.rs begins here========================\n");

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Initial Vector: {:?}", numbers);

    // get element at particular index
    println!("Element at index 2 is: {}", numbers[2]);

    // Making Vectors mutable
    let mut mutable_vector: Vec<i64>= vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    println!("Mutable 64 bit number in the tuple is at index: {:?}", mutable_vector[8]);

    // Re-assign values
    mutable_vector[2] = 48720535749569854;
    println!("Updated vector after replacing element at particular index: {:?}", mutable_vector);

    // Add on to vector only applicable on mutable vectors
    mutable_vector.push(6);
    println!("Updated vector after pushing in element: {:?}", mutable_vector);

    // Pop on to vector only applicable on mutable vectors
    mutable_vector.pop();
    println!("Updated vector after poping out element: {:?}", mutable_vector);

    // to check the length of vector
    println!("Vector Length of mutable_vector: {} \nVector length of numbers: {}", mutable_vector.len(), numbers.len());

    // vectors are stacked allocated
    // println!("mutable_vector vector occupies {} bytes \nnumbers vector occupies {} bytes", std::mem::size_of_val(&mutable_vector), std::mem::size_of_val(&numbers));  //we can call the library std::mem globally
    println!("mutable_vector vector occupies {} bytes \nnumbers vector occupies {} bytes", mem::size_of_val(&mutable_vector), mem::size_of_val(&numbers));

    // Get slice of vector
    let slice: &[i32]= &numbers[0..2];
    println!("Slice of vector numbers from index 0 to 2 is: {:?}", slice);

    // Loops through vector value
    for x in mutable_vector.iter(){
        println!("Number: {}", x);
    }

    //Loop and mutate values           //similar to javascript .map higher order array
    for x in mutable_vector.iter_mut(){
        *x *= 3;
    }
    println!("{:?}", mutable_vector);

    // End of vectors
    println!("=========================Vectors.rs starts here========================\n");
}   

