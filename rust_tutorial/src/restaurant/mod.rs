use self::pizza_order::Pizza;

mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String
    }

    impl Pizza {
        fn lunch(topping: &str) -> Pizza{
            return Pizza{
                dough: String::from("Regular dough"),
                cheese: String::from(" mezarella"),
                topping: String::from(topping),
            }
        }

        pub fn string(&self) -> String{
            let mut s: String = String::new();

            s.push_str(&self.dough);
            s.push('-');
            s.push_str(&self.cheese);
            s.push('-');
            s.push_str(&self.topping);
            return s;
        }
    }

    pub fn order() -> Pizza{
        return Pizza::lunch("Veggies")
    }
}

pub fn order_pizza() -> Pizza {
    crate::restaurant::pizza_order::order()
}
