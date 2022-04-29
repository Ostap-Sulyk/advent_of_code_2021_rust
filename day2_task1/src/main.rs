#![allow(unused)]
struct Command<'a> {
    direction: &'a str,
    value: &'a str,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }
    fn forward(&mut self, amount: i32) {
        self.x += amount;
    }
    fn down(&mut self, amount: i32) {
        self.y += amount;
    }
    fn up(&mut self, amount: i32) {
        self.y -= amount;
    }
    fn get_position(&self) -> i32 {
        self.x * self.y
    }
}

fn main() {
    let mut position = Position::new();

}
