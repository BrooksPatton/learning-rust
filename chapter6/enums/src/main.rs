// This is not a good example of enums, but rather it would be better as a struct.

// enums can only be one of the values defined, not both at the same time.

#[derive(Debug)]
enum Game {
    Name(String),
    System(String)
}

impl Game {
    fn create(name: &str) -> Game {
        Game::Name(name.into())
    }
}

fn main() {
    let mut mario = Game::create("Mario Brothers");
    mario = Game::System("NES".into());

    println!("{:?}", mario);
}
