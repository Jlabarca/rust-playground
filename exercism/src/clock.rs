use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = ((hours + (minutes as f64 / 60.0).floor() as i32) % 24 + 24) % 24;
        let minutes = (minutes % 60 + 60) % 60;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    let clock = Clock::new(10, 0).add_minutes(3);
    println!("{:?}", Clock::new(15, 37));
    println!("{:?}", Clock::new(15, 37).to_string());
    assert_eq!(clock.to_string(), "10:03");
    assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
}
