// Arrays are Fixed list where elements are the same data types
use std::mem;
pub fn arrays(){
    // Beginning of arrays.rs
    println!("=========================Arrays.rs starts here========================\n");

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // get element at particular index
    println!("Element at index 2 is: {}", numbers[2]);

    // Making arrays mutable
    let mut mutable_array: [i64; 10]= [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    println!("Mutable 64 bit number in the tuple is at index: {:?}", mutable_array[8]);

    // Re-assign values
    mutable_array[2] = 48720535749569854;
    println!("Updated array: {:?}", mutable_array);

    // to check the length of array
    println!("Array Length of mutable_array: {} \nArray length of numbers: {}", mutable_array.len(), numbers.len());

    // Arrays are stacked allocated
    // println!("mutable_array Array occupies {} bytes \nnumbers Array occupies {} bytes", std::mem::size_of_val(&mutable_array), std::mem::size_of_val(&numbers));  //we can call the library std::mem globally
    println!("mutable_array Array occupies {} bytes \nnumbers Array occupies {} bytes", mem::size_of_val(&mutable_array), mem::size_of_val(&numbers));

    // Get slice of array
    let slice: &[i32]= &numbers[0..2];
    println!("Slice of array numbers from index 0 to 2 is: {:?}", slice);

    // end of print.rs
    println!("==========================Print.rs ends here=========================\n");

}   

