use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Rect {
    x: i64,
    y: i64,
}

impl Rect {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }

    fn area(&self) -> i64 {
        self.x * self.y
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}={}", self.x, self.y, self.area())
    }
}

fn main() {
    let r = Rect::new();
    println!("{}", r);

    let serialized = serde_json::to_string(&r).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Rect = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
}
