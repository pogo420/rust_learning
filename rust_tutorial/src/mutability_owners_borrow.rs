
fn try_change(data: &mut String){
    data.push_str("ola bam");
}



pub fn mutability() -> (){

    let mut sentense: String = String::from("dummy sentense is here");
    println!("Before: {}",sentense);

    try_change(&mut sentense);
    println!("After: {}",sentense);
}