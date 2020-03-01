// Variables hold primitive data or references to data
// Variabvles are immutable by default
// Rust is a bloc-scoped language


pub fn run() {
    let name = "Brad"; 
    let mut age = 37;  // mut de mutable, de lo contrario funciona similar a const en js.
    println!("My name is {} and i am {} years old", name, age);
    age = 25;
    println!("My name is {} and i am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;  // i32 = integer 32-bits
    println!("ID:{}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("brad", 37);
    println!("{} is {}", my_name, my_age);
}