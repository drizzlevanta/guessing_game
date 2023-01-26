//Error handling - Custom types for validation
//a new type and put the validations in a function to create an instance
//use this new type to do bulk validation
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("value should be between 1 and 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
