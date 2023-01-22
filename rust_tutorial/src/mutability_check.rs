struct A{
    name: String
}

pub fn check_vec_mut(){
    let mut vec: Vec<A> = Vec::new();
    let x = A{name: String::from("String1")};
    vec.push(A{name: String::from("String1")});
    vec.push(x);

    println!("{}", vec[1].name);

    vec[1].name=String::from("String3");

    println!("{}", vec[1].name);
}

