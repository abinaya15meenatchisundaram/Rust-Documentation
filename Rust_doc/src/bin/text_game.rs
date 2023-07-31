use std::io;

// Define the enum for different rooms in the cave
enum Room {
    Entrance,
    TreasureRoom,
    MonsterRoom,
    PuzzleRoom,
    Exit,
}

// Define the struct for the player
struct Player {
    name: String,
    health: i32,
}

impl Player {
    // Function to initialize a new player
    fn new(name: String) -> Self {
        Player {
            name,
            health: 100,
        }
    }

    // Function to handle player damage
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health < 0 {
            self.health = 0;
        }
    }
}

fn main() {
    println!("Welcome to the Mysterious Cave!");

    // Get player's name
    println!("Please enter your name:");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read input");
    let player_name = input_name.trim().to_string();

    // Create a new player
    let mut player = Player::new(player_name);

    // Start the game in the entrance room
    let mut current_room = Room::Entrance;

    loop {
        match current_room {
            Room::Entrance => {
                println!("You are in the entrance room of the cave.");
                println!("You see two paths ahead. (1) Go left, (2) Go right");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let choice = input.trim().parse::<i32>().unwrap_or(0);

                match choice {
                    1 => current_room = Room::MonsterRoom,
                    2 => current_room = Room::PuzzleRoom,
                    _ => println!("Invalid choice. Please try again."),
                }
            }
            Room::MonsterRoom => {
                println!("You have encountered a fearsome monster!");
                println!("What will you do? (1) Fight, (2) Flee");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let choice = input.trim().parse::<i32>().unwrap_or(0);

                match choice {
                    1 => {
                        println!("You attack the monster!");
                        // Simulate battle by reducing player's health
                        player.take_damage(30);
                        println!("You took some damage. Your health is now: {}", player.health);
                    }
                    2 => {
                        println!("You decide to flee and go back to the entrance room.");
                        current_room = Room::Entrance;
                    }
                    _ => println!("Invalid choice. Please try again."),
                }
            }
            Room::PuzzleRoom => {
                println!("You have entered a room with a mysterious puzzle.");
                println!("Solve the riddle to proceed:");
                println!("I speak without a mouth and hear without ears. I have no body, but I come alive with the wind. What am I?");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let answer = input.trim().to_lowercase();

                if answer == "echo" {
                    println!("Congratulations! You solved the puzzle and gained some treasure!");
                    // Simulate treasure acquisition by increasing player's health
                    player.health += 20;
                    println!("Your health is now: {}", player.health);
                    current_room = Room::TreasureRoom;
                } else {
                    println!("Incorrect answer. Try again.");
                }
            }
            Room::TreasureRoom => {
                println!("You have found the treasure room!");
                println!("You have successfully completed the adventure!");
                println!("Your final health: {}", player.health);
                break;
            }
            Room::Exit => break,
        }
    }
}