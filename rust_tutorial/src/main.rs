#![allow(unused)] 
use core::slice;
use std::ffi::c_void;
use std::sync::mpsc;
/* 
Suppress waring of unused variables 
1. Use the above compiler directives
2. If a variable has underscore prefix it's ignored if unsued.
*/ 
use std::{io, ops::Range};
use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;

// using modules
mod restaurant;
use restaurant::order_pizza;

mod optionals; // each file is actually a module
use optionals::is_opt;
use optionals::is_err;

mod traits_check;
use traits_check::{AnimalSound, Horse, Pig};


mod custom_exception;
use custom_exception::error_gen_1;
use custom_exception::error_gen_2;

mod mutability_check;
use mutability_check::check_vec_mut;

mod concurrency_concept;
use concurrency_concept::{check_thread, process_url};
use std::thread;

mod  mutability_owners_borrow;
use mutability_owners_borrow::mutability;


mod method_function;
use method_function::Coordinate;

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

// Returning a unit type, implies return nothing.
fn char_test(c : char) -> (){
    println!("{}", c);
}


fn slices() {

    // mutable slice
    let mut ar1 = [1, 2, 3, 4, 5];
    let ar_slice: &mut [i32] = & mut ar1[1 .. 3];
    
    // immutable slice
    let ar2: [i32; 5] = [1, 2, 3, 4, 5];
    let ar_slice_2:  &[i32] = &ar2[1 .. 3];


    println!(" mutable {}", ar_slice[0]);
    println!("immutable {}", ar_slice_2[0]);

    ar_slice[0] = 8;
    println!("changed mutable {}", ar_slice[0]); // changed 

}

fn tuples_check() {

    // We can't get length of tuple.

    let data: (&str, u8) = ("hello rust", 8);

    // tuple access
    println!("{}", data.0);
    println!("{}", data.1);
}

fn struct_check() {

    struct  User {
        name: String,
        age: u8,
    };

    // immutable reference
    let u1 = User{
        name: String::from("Gablu Mukherjee"),
        age: 34,
    };

    // mutable reference
    let mut u2 = User{
        name: String::from("Puglu Mukherjee"),
        age: 34,
    };


    println!("{}",u1.age);
    u2.age = 98;


    // Tuple struct 
    struct Point(i8, i8, i8);
    let p1 = Point(12, -9, 7);

    println!("{}/{}/{}", p1.0, p1.1, p1.2);

}


fn enum_check(){

    enum AgeType {

        Baby,
        Child,
        Teen,
        Adult,
        Mid,
        Elder
    }

    let gablu: AgeType = AgeType::Adult;

    let puglu: AgeType = AgeType::Baby;

}

fn have_liquor(age: u8) -> Option<u8> {

    if age < 18 {
        return None;
    }
    else {
        return Some(1);
    }
}


fn check_options(){

    // accessing data from option
    if let Some(data) = have_liquor(19){
        println!("{}", data);
    }
}


enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}


fn check_in_cents(coin: Coin) -> u8{
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 100, // default
    }
}


fn vector_check() {

    // empty string vector
    let mut names: Vec<String> = vec![];

    names.push(String::from("Puglu"));
    names.push(String::from("Gablu"));
    names.push(String::from("Gublu"));

    for name in names{
        println!("{}", name);
    }

}

fn check_hash_table() {

    let mut persons: HashMap<&str, u8> = HashMap::new();
    
    persons.insert("ola", 34);
    persons.insert("sam", 4);

    if persons.contains_key("ola"){
        println!("{}", persons["ola"])
    }

    for (n, s) in persons{
        println!("{}=>{}", n, s);
    }
}

fn check_t(){

    let i :u8 = 'a' as u8;
    println!("{}",i);
}


// entry point of rust programs
fn main() {

    let others = false;
    if others {
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
    Unique feature: match, substitutes switch
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

    /* String data type: 
    string two types:
    1. String -> Mutable in heap memory.
    2. &str -> Immutable for stack/static memory.
    Called a "string slice" or just a "slice". 
    A slice is just a view onto some data, and that data can be anywhere

    We use str reference if we need to view the data. -- string(1)
    */
    let mut my_string: String = String::new();
    my_string.push('A');
    my_string.push('\n');
    my_string.push_str("ola boom kolo");

    // string(1)
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


    /*
    Ownership is an important concept.
    1. Each value has a variable and its called owner.
    2. Their is only one owner at a time.
    3. When the owner goes out of scope it disappears.
    */
    let s: String = String::from("Loko is here");
    // let s1: String = s;
    //println!("{}", s)  // s no longer valid
    // We need to clone it to use it
    let s1: String = s.clone();
    println!("{}", s);  // s is valid as its cloned



    /*
    HasMap: Key value
    */
    let mut map: HashMap<u8, &str> = HashMap::new();

    map.insert(1, "ondu");
    map.insert(2, "erudu");
    map.insert(3, "mooru");

    for (k, v) in map.iter(){
        println!("{}-{}", k, v);
    }

    println!("Length of map: {}", map.len());

    // defining values in one shot
    let dict: HashMap<&str, &str> = HashMap::from([
        ("ola", "cab"),
        ("pom", "pompm")
    ]);

    for (k, v) in dict.iter(){
        println!("{}-{}", k, v);
    }


    /*
    Struct + Traits : Can be used for creating adaptors
    Struct -> c like. Class.
    Traits -> function implementation.
    */

    trait DbAdaptor {
        fn new(user: String, pass: String, url: String) -> Self;
        fn connec(&self) -> String;
        fn alive(&self) -> u8;
    }


    struct Dbx {
        user_name: String,
        password: String,
        url: String
    }

    struct DbA {
        user_name: String,
        password: String,
        url: String
    }

    impl DbAdaptor for Dbx {

        fn new(user: String, pass: String, url: String) -> Self {
            return Dbx{user_name: user, password: pass, url: url};
        }

        fn connec(&self) -> String {

            let mut s: String = String::new();
            s.push_str(&self.url);
            s.push('-');
            s.push_str(&self.user_name);
            s.push('-');
            s.push_str(&self.password);
    
            return s;
        }

        fn alive(&self) -> u8 {
            return 2;
        }
    }

    impl DbAdaptor for DbA {

        fn new(user: String, pass: String, url: String) -> Self {
            return DbA{user_name: user, password: pass, url: url};
        }

        fn connec(&self) -> String {

            let mut s: String = String::new();
            s.push_str(&self.url);
            s.push('*');
            s.push_str(&self.user_name);
            s.push('*');
            s.push_str(&self.password);
    
            return s;
        }

        fn alive(&self) -> u8 {
            return 4;
        }
    }

    let dbx: Dbx = DbAdaptor::new("ola".to_string(), "Sam".to_string(), "dbx.url.co.in".to_string());
    let dba: DbA = DbAdaptor::new("ola".to_string(), "Sam".to_string(), "dba.url.co.in".to_string());

    println!("Dbx: {}-{}", dbx.connec(), dbx.alive());
    println!("Dba: {}-{}", dba.connec(), dba.alive());

    // traits revisit
    // Based on the variable typle associated implementation is used
    let horse: Horse = AnimalSound::new();
    let pig: Pig = AnimalSound::new();

    println!("{}-{}", horse.get_sound(), pig.get_sound());



    let p = order_pizza();
    println!("{}",p.string());


    /*
    Closures: High level functions

    let f = |parameters|-> return_type {BODY}
    */


    let f = |i: u8, s: &str|-> String {
        let mut s1 = String::new();
        s1.push_str(&s);
        s1.push_str(&(i.to_string()));
        return s1;
        };

        println!("{}", f(32, "Ola"));


    // Format creates a string
    let s = format!("Hello {} How are you?, you have INR:{}", "Sam", 45);
    println!("{}", s);

    /* 
    * Optional and Result operators to manage exception in rust.
    */

    if is_opt("X".to_string()).is_some(){
        println!("{}", is_opt("M".to_string()).unwrap())
    }else{
        println!("No data seen!!")
    }

    let g = is_err("G".to_string());

    match g {
        Ok(b) => println!("Valid gender: {}", b),
        Err(e) => println!("Invalid gender:{}",e)
    }

    println!("Using if else--");

    let g = is_err("G".to_string());
    if g.is_err(){
        println!("Invalid data:{}",g.unwrap_err());
    }else{
        println!("Valid data:{}",g.unwrap());
    }

    /*
    * Custom exceptions
    */

    if error_gen_1(1).is_err(){
        println!("Custom error: {}", error_gen_1(1).unwrap_err());
    }

    if error_gen_2(1).is_err(){
        println!("Custom error: {}", error_gen_2(1).unwrap_err());
    }


    /*
    Checking mutability concept

    Mutability is a property of either a borrow (&mut) or a binding (let mut). 
    This means that, for example, you cannot have a struct with some fields mutable and some immutable.
    If a complex data type is binded as mut then nested data types are mut as well.
    */

    check_vec_mut();


    /* 
    Thread logic 
    */

    let handle = check_thread();
    handle.join().unwrap(); // waits for all spawned thread to complete after main thread is completed
    // Add it immediately after the span thread so that all the spanned will be closed before proceeding further.

    /*
    mpsc -> multi producer / multi consumer
    */
    let (tx, rx) = mpsc::channel();

    let urls = vec![
        String::from("ola"),
        String::from("Hello"),
        String::from("Hi"),
        String::from("Namaste")
    ];

    for i in urls{
        let tx2 = tx.clone();
        thread::spawn( move || {
                process_url(i, tx2)  // we are sending the tx, after processing we are sending data via tx from thread.
            });
    }

    drop(tx); // manually drop the main sender, sender in spawnned threads are dropped automatically

    for r in rx{
        println!("Data received: {}", r);
    }

    /*
    Shared memory 
    Mutex - only one can access at a time.

    */


    /*
    rangees 
    */

    // range short hand
    for i in 1..5{
        println!("Data {}",i)
    }

    // range actual implementation
    let r:Range<u8> = Range{start: 1 , end: 5};
    for i in r{
        println!("Range actual: {}", i);
    }

    char_test('p');

    mutability();

    slices();
    tuples_check();
    struct_check();
    check_options();


    println!("{}",check_in_cents(Coin::Dime));


    let c = Coordinate::new(12, 13);

    println!("{}", c.sum())

    }

   


    // vector_check();
    check_hash_table();

}
