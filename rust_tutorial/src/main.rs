#![allow(unused)] // suppress waring of unused variables
use std::{io, ops::Range};
use rand::Rng;
use std::cmp::Ordering;


fn my_fun(i: u32, j: u32) -> u32 {
    return i+j;
}

fn get_2(i: u32) ->(u32, u32) {
    // function returning multiple arguments
    return (i+1, i+2);
}

fn sum_list(list: &[i32]) -> i32 {
    // function receiving list 
    let mut sum=0;
    for i in list {
        sum += i;
    }
    return sum;
}

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

    let is_boy: bool = true;
    println!("bool: {}",is_boy); // need to convert to string before printing

    let mut grade: char = 'A';  // char datatype
    println!("char: {}",grade);  // string conversion

    let i: i32 = rand::thread_rng().gen_range(1..101);

    println!("val: {}",i);


// conditional check
let age_value: u8 = 19;

if age_value >= 1 && age_value <= 18 {
    println!("Teens..!");
} else if  age_value > 18 && age_value <= 29 {
    println!("Twenties..!");
} else {
    println!("We are cool..!");
}

// ternary simulation 
let can_vote: bool = if age_value > 18 { 
        true 
    } else {
        false
    };
    println!("can vote: {}", can_vote);

/*
Unique feature match, substitutes switch
*/

let person_age:u8 = 20;
let voting_age:u8 = 18;

// static version 
match person_age {
    1..=18 => println!("Can not vote-S"),
    19..=u8::MAX =>  println!("Can vote-S"),
    _ =>  println!("Not sure-S"),
}

//runtime version
match person_age.cmp(&voting_age) {
    Ordering::Less => println!("Can't vote-R"),
    Ordering::Greater => println!("Can vote-R"),
    Ordering::Equal => println!("Can vote-R"),
}



/*
Array in rust
1. Homogeneous 
2. Fixed size
*/

let array1: [u8; 4] = [1,2,3,4];
println!("array value:{}", array1[2]);
println!("array length:{}", array1.len());

// while loop
let mut loop_inx: usize= 0; // as loop variable will change, it's defined mutable

while loop_inx < array1.len() {
    println!("value: {}",  array1[loop_inx]);
    loop_inx += 1;
}

// for loop for array

for val in array1.iter() {
    println!("val: {}", val);
}


/*
Tuple: (Fixed sized + Heterogeneoues) data 
*/
let my_tuple: (u8, String, f64) = (23, "Ola".to_string(), 3.14);
println!("{}", my_tuple.0);
/* string two types:
1. String 
2. &str

*/
let mut my_string: String = String::new();
my_string.push('A');
my_string.push('\n');
my_string.push_str("ola boom kolo");


for word in my_string.split_whitespace(){
    println!("{}", word);
}


/*
Enums are custom data types.
Similar to C enums.
We can define methods for them.
*/
enum Days {
    Workday,
    Weekend,
    Holiday
}

impl Days {
    fn is_office(&self) -> bool {
        match self {
            Days::Holiday | Days::Weekend => false,
            _ => true,
        }
    }
}


let today:Days = Days::Holiday;
println!("{}", today.is_office());


/*
Vectors:
1. Can grow if mutable.
2. Homogeneious data types.
*/

let mut my_vect: Vec<u8> = Vec::new(); // initializing empty vector

// growing data
for i in 1..10 {
    my_vect.push(i);
}
// getting value by index 
println!("{}", my_vect[2]);

for i in my_vect{
    println!("{}", i);
}


println!("{}", my_fun(2, 8));
let (v1, v2) = get_2(40);

println!("{} and {}", v1, v2);


let l:Vec<i32> = vec![1,34,89,45];
println!("{}", sum_list(&l));


}
