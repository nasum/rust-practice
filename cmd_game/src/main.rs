use std::io;

struct Player {
    name: String,
    life: i32,
}

impl Player {
    fn create(name: String) -> Player {
        Player { name, life: 100 }
    }
}

fn main() {
    println!("Game start!");

    loop {
        println!("Please input your name.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line");
        let player = Player::create(input.trim().to_string());

        println!("Hi, {}!", player.name);
        println!("Your life is {}.", player.life);
    }
}
