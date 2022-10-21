pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        return User {
            name: name,
            age: age,
            weight: weight,
        };
    }

    pub fn name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn age(&self) -> u32 {
        return self.age;
    }

    pub fn weight(&self) -> f32 {
       return self.weight;
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    // Returns: a User with name "Bob", age 32, and weight 155.2

    bob.weight();
    println!("{}", bob.weight());
    // Returns: 155.2

    bob.set_age(33);
    println!("{}", bob.age);
    // Updates Bob's age to 33; happy birthday Bob!
}

