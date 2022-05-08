// Arrays are Fixed list where elements are the same data types
pub fn arrays(){
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
}   