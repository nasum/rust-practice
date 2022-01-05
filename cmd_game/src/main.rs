use std::io;

struct Player {
    name: String,
}

fn main() {
    println!("Game start!");

    loop {
        println!("Please input your name.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read line");
        let player = Player {
            name: input.trim().to_string(),
        };

        println!("Hi, {}!", player.name);
    }
}
