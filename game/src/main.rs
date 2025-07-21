use std::io::{self, Write};
use std::collections::HashMap;

mod data;

fn get_input(prompt: &str) -> String{
    // println!("{}",prompt);
    let mut stdout = io::stdout();
    let _ = stdout.write_all(prompt.as_bytes());
    let _ = stdout.flush();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn main() {
    let mut components = data::Components {
        current_entities: 0,
        names: HashMap::new(),
        descriptions: HashMap::new(),
        destinations: HashMap::new(),
        locations: HashMap::new(),
    };

    let mut state = data::State {
        current_location: 0,
    };
    data::main::load_data(&mut components, &mut state);

    loop {
        println!("You are at {}", components.names.get(&state.current_location).unwrap());
        println!("You see {}", components.descriptions.get(&state.current_location).unwrap());
        let entity_ids = components.locations.get(&state.current_location).unwrap();
        for entity_id in entity_ids {
            println!("{}", components.names.get(entity_id).unwrap());
        }
        let input: String = get_input("> ");
        if input == "quit" {
            println!("Goodbye!");
            break;
        }
    }
}
