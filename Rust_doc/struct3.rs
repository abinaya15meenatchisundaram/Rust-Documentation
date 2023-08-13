struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };
    let person1 = Person {
        name: String::from("Alan"),
        age: 45,
    };
    person.say_hello();
    person1.say_hello();
}