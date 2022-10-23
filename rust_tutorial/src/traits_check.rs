pub trait AnimalSound {
    fn new() -> Self;
    fn get_sound(&self) -> String;
}

pub struct Pig{
}

pub struct Horse{
}


impl AnimalSound for Pig {
    fn new() -> Self{
        return Pig{};
    }
    fn get_sound(&self) -> String{
        return String::from("Pii");
    }
}

impl AnimalSound for Horse {
    fn new() -> Self{
        return Horse{};
    }
    fn get_sound(&self) -> String{
        return String::from("Chii");
    }
}

