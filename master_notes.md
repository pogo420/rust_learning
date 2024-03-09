# Rust learning notes.

## Ownership:
* compile time rules for memory management.
* There are three rules:
   * Every data in rust has a owner.
   * There can be only one owner of a data.
   * Owner out of scope -> Data is deleted.
* Heap allocation if size id not known or it can grow or shrink.
   * Type `String` is stored in heap.
   * All heap allocated data are moved not copied unlike stack allocated data.
   * Heap allocated data is moved to present dangling pointers(Only one owner).
   * Dangling pointer, a pointer pointing to address that is not valid.

* Usefulness of ownership:
   * Prevents memory safety issues, like
      * Dangling pointer: Assessing a invalid address.
      * Double free: Freeing already free memory.
      * Memrory leaks: Not freeing memory.

* Check this wiki for more [details](https://github.com/pogo420/rust_learning/wiki/Rust-ownership-and-borrowing).   

## Borrowing:
* Two rules of borrowing:
   * Either 1 mutable reference or any number of immutable references. NOT BOTH.
   * References must be always valid.

## Mutability:
* By default all data in rust is immutable.
* To modify post defining we need to mention it mutable, `mut`.
    * Example:   `let mut x: u8 = 2;`
* Rerefence:
   * irrespective of data, by default all references are immutable by default.
   * Only mutable data can have mitable reference, We need to mention it explicitly.
* Wiki [reference](https://github.com/pogo420/rust_learning/wiki/Mutability)

## desrtucturing assignments:
```
(.., x) = (3, 4) // 3 will be ignored
[x, ..] = [8, 9] // 9 will be ignored
```

## numbers
* All size in bits below:
   * signed -> i8, i16, i32, i64, i128
   * unsigned -> u8, u16, u32, u64, u128
   * usize and isize are architecture dependent,
      * 32 bit processor -> 32bit
      * 64 bit processor -> 64bit
      * usize pointer can hold any memory address in a computer.
    * floating point:
       * f32 and f64

## ranges
```
// range short hand
for i in 1..5{
    println!("Data {}",i)
}

// range actual implementation
let r:Range<u8> = Range{start: 1 , end: 5};
for i in r{
    println!("Range actual: {}", i);
}
```

## characters:
* example:
` let c: char = 'a'`
*  Size = 32 bits/4 bytes
* In rust, `char` can hold any unicode value.
* Notice the single quote.

## bool
* bool -> `true` or `false`
* size -> one bytes.

## unit type
* Its a empty tuple of size zero.
* Its used to return nothing from function.
* We don't have to mention explicitly.
```
// here return is unit type
fn functio_unit_type_return() -> (){
    println("Ola 32");
}
```

## panic:
* In rust we can immediately terminate a program with panic/error via `panic!()` macro.

## Compound types:
### String and &str:
* comparison:

| String | &str |
|---|---|
|Mutable | Imutable, just for viewing |
| Allocated in heap | Allocated in stack |
| Heavy | Light wight and efficient |

* Example:
```
let data: &str = "hello  rust"  # stack storage its a string literal
let data_m: String = String::from("hello in heap");

let s1 = &s[0:4]; // delivers a immutable view to heap.
```

* Why str is not valid but &str is a valid type?
   * size of str is not know and don't have fixed size but reference to a str will have fixed size.

### Slice
* Reference to a contiguous sequence of elements in a collection.
* We need to use references of slice as size is not known in run time.
* We can borrow part of data; We can mutate as well. BUT no ownership transfer.
* Example:
```
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
```

### Tuple
* Heterogenious, contiguous, fixed size.
* Defined in compile time - Stoted in stack.
```
fn tuples_check() {

    // We can't get length of tuple.

    let data: (&str, u8) = ("hello rust", 8);

    // tuple access
    println!("{}", data.0);
    println!("{}", data.1);
}
```

### Struct
* Compound data type, most important in programming world.
* Most similar to class.
```
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
```

### Enum:

* Named data.
```
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
```

### Option
* Its a enum for handling function return type as optional.
* Function can return data of desired type or it can return None.
```
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
```

## Pattern match 
* match clause:
```
fn check_in_cents(coin: Coin) -> u8{
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 100, // default
    }
}
```

* if let clause(All data options not required):
```
if let Some(data) = have_liquor(19){
        println!("{}", data);
    }
```

## Methods and Function 
* Both imlpemented using `impl` keyword.
```
pub struct Coordinate{
    pub x_axis: u8,
    pub y_axis: u8,
}

impl Coordinate{
    // self => object reference (Method)
    pub fn sum(&self) -> u8 {
        return self.x_axis + self.y_axis;
    }

    // No reference => (Function) 
    pub fn new(x_axis: u8, y_axis: u8) -> Coordinate {
        return Coordinate {
            x_axis,
            y_axis,
        };
    }

}
```

## Generics

* Placeholders for generic types.
* In struct 
```
struct check_gen<T>{
    x: T,
    y: T,
}
```
* In function, it using generics and takes a struct
```
fn check_gen_fn<T>(s: check_gen<T>){
    ....
    ....
}
```

## Traits:

* Set of functions implemented by a type.
* Its a set of deficitions that a type will implement.
* It's similar to class.
* check the [example](./rust_tutorial/src/traits_check.rs).
* traits are automatically implemented for a struct or enum by compiler by the rust compiler.
* Trait as function:
```
// Gurantees item will be any item that implements Summary trait; 
// So it insures the object will have `summarize()` method.
pub fn notify(item: &impl Summary){
    item.summarize();
}
```
* another syntax of above is(called Trait bound):
```
pub fn notify<T: Summary>(item: &T){
    item.summarize();
}
```

* We can combine traits with `+` operator.
```
Option 1:
fn sm_func<T: Display + Clone, U: Clone + Debug>(t : &T, u: &u) -> i32 {
    ....
}

Option 2:
fn sm_func<T, U>(t : &T, u: &u) -> i32 
where
    T: Display + Clone
    U: Clone + Debug
    {
    ....
}
```

* Trait as function return type.
```
// Gurantees that the function will return an item that immplements Animal trait
fn some_func() -> impl Animal {
    ....
}
```



## Vectors:
* Dynamically sized arrray.
* Allocated in heap.
* String is a Vec<u8>.
* Example:

```
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
```

## HashMap:
* Key Value pair
* Allocated at heap.
* key is hashed.
* efficient lookup.
* Example:

```
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
```

## panic:
* Panic will printout error message => unwind the stack => exit the stack.
* In case of MT program; i will exit current thread not whole program.
* Panic `panic!()`

## Result/Option:
* Enum type for handling errors.
* Details, [click](./rust_tutorial/src/optionals.rs).

## Display/Debug:
* If any object implements `Debug` and `Dislay` trait, we can print them.
* print!("{}", object); // methods of Debug trait is called.
* print!("{:?}", object); // methods of Display trait is called.

## Iterator:
* Trait with implements the `next()` method.

## Cargo:
* Cargo -> Manager for build, testing and execution.
* Program as saved in [cargo.io](https://crates.io/)


## Topic to cover in details later:
* Explore trait pointers.
* static dispatch and dynamic dispatch.
* vtable.
* Box, Box with dyn.
