#![allow(unused)] // suppress waring of unused variables
use std::io;

// entry point of rust programs
fn main() {
    println!("Enter your name pal!"); // its a macro, check the exclamation mark
    /*
    All variable delaration must start with let
     */
    let mut name: String = String::new(); // mutable variable, by default all variables are immutable
    let greetings: &str = "All cool boss?";
    
    // taking input
    io::stdin().read_line(&mut name).expect("did not recieve value");
    println!("Hello {}!, {}", name.trim(), greetings);  // python like format


    const PI: f32 = 3.14; // defining constants, 32 bit 

    // same variable name different data types 
    let age: &str = "31";

    // string to integer conversion
    let mut age: u32 = age.trim().parse().expect("Did not receive integer!");
    age+=1;

    println!("Hello {}! you age is {}",name.trim(), age);
    // unsiged u8, u16, u32, u64, u128
    // siged i8, i16, i32, i64, i128
    println!("Max u8 is {}", u8::MAX);
    println!("Max i8 is {}", i8::MAX);
}
