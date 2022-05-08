// Conditionals - used to check the condition of something and act on the result
pub fn conditionals(){
    // Beginning of conditionals
    println!("=========================conditionals.rs starts here========================\n");

    let age = 18;
    let check_ID: bool = false;
    let learning_License: bool= true;

    // If / Else
    if age >= 18 && check_ID{
        println!("You are applicable to apply for Driving license in India");
    }
    else if age < 18 && check_ID && learning_License{
        println!("You can still drive");
    }
    else if age<18 && check_ID{
        println!("Sorry, You are underage")
    }
    else{
        println!("make sure your ID is checked")
    }

    // Shorthand If
    let is_of_age = if age>=18{true} else{false};
    println!("Is an Adult: {}", is_of_age);

    // end of line
    println!("=========================Conditionals.rs ends here========================\n")
}