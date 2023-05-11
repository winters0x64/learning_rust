use std::io::{self, Write};

struct Room {
    name: String,
    description: String,
    north: Option<usize>,
    south: Option<usize>,
    east: Option<usize>,
    west: Option<usize>,
    items: Vec<String>,
}

impl Room {
    fn new(name: &str, description: &str) -> Room {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            north: None,
            south: None,
            east: None,
            west: None,
            items: Vec::new(),
        }
    }

    fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }
}

fn main() {
    let mut rooms = vec![
        Room::new("Entrance", "You are standing at the entrance to a dark and mysterious castle. The door behind you slams shut with a loud clang."),
        Room::new("Hallway", "You are in a long, dimly lit hallway. The walls are covered in cobwebs and the air is thick with dust."),
        Room::new("Library", "You are standing in a vast library filled with dusty books and ancient scrolls. The shelves stretch up to the ceiling, and there are hidden passages everywhere."),
        Room::new("Armory", "You have entered an armory filled with swords, shields, and armor. There is a musty smell in the air."),
        Room::new("Throne Room", "You have entered the throne room of the castle. The room is vast and imposing, with a giant throne at the far end."),
    ];

    // Define the connections between rooms
    rooms[0].east = Some(1);
    rooms[1].west = Some(0);
    rooms[1].north = Some(2);
    rooms[1].south = Some(3);
    rooms[2].south = Some(1);
    rooms[3].north = Some(1);
    rooms[3].east = Some(4);
    rooms[4].west = Some(3);

    // Add some items to the rooms
    rooms[2].add_item("Ancient Scroll");
    rooms[3].add_item("Rusty Sword");

    // Start the game in the first room
    let mut current_room = 0;

    loop {
        // Print the current room description
        println!("{}:", rooms[current_room].name);
        println!("{}", rooms[current_room].description);
        if !rooms[current_room].items.is_empty() {
            println!("You see the following items:");
            for item in &rooms[current_room].items {
                println!(" - {}", item);
            }
        }

        // Get user input
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        // Handle user input
        match input.as_str() {
            "north" | "n" => {
                if let Some(room) = rooms[current_room].north {
                    current_room = room;
                } else {
                    println!("You can't go that way!");
                }
            }
            "south" | "s" => {
                if let Some(room) = rooms[current_room].south {
                    current_room = room;
                } else {
                    println!("You can't go that way!");
                }
            }
            "east" | "e" => {
                if let Some(room) = rooms[current_room].east {
                    current_room = room;
                } else {
                    println!("You can't go that way!");
                }
            }
            "west" | "w" => {
                if let Some(room) = rooms[current_room].west {
                    current_room = room;
                } else {
                    println!("You can't go that way!");
                }
            }
            "quit" | "exit" => {
                println!("Thanks for playing!");
                break;
            }
            _ => {
                println!("I don't understand that command.");
            }
        }
    }
}