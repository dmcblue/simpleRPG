use std::io::{self, Write};
use std::collections::HashMap;

mod data;

struct State {
    pub current_location: usize,
}

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
    return input.trim().to_string();
}

fn main() {
    let mut components = data::Components {
        current_entities: 0,
        names: HashMap::new(),
        descriptions: HashMap::new(),
        destinations: HashMap::new(),
        locations: HashMap::new(),
    };

    let mut state = State {
        current_location: data::main::get_start_location_id(),
    };
    data::main::load_data(&mut components);

    loop {
        let scene = get_scene(& components, & state);
        render(& scene, & state, & components);
        let input: String = get_input("> ");
        match update(input, & scene, &mut state, & components) {
            Err(s) => { println!("{}", s); break; },
            _ => ()
        }
    }
}

fn update<'a>(input: String, scene: &'a Scene, state: &'a mut State, components: &'a  data::Components) -> Result<(), &'a str> {
    if input == "quit" {
        return Err("Goodbye!");
    } else {
        let mut i: usize = input.parse::<usize>().unwrap();
        i = i - 1;
        let action = &scene.actions[i];
        match action.action_type {
            ActionType::GO => state.current_location = *components.destinations.get(&action.arg_1.unwrap()).unwrap(),
            ActionType::LOOK => render_detailed(& scene, & state, & components),
            // ActionType::TALK =>
            _ => ()
        }
    }
    return Ok(());
}
enum ActionType {
    GO,
    LOOK,
    TALK
}
// #[derive(Default)]
struct Action {
    action_type: ActionType,
    arg_1: Option<usize>,
    arg_2: Option<usize>,
    arg_3: Option<usize>,
}
// #[derive(Default)]
struct Scene<'a> {
    location_id: usize,
    entity_ids: &'a Vec<usize>,
    exit_ids: Vec<&'a usize>,
    actions: Vec<Action>,
}
impl Default for Action {
    fn default() -> Action {
        Action {
            action_type: ActionType::LOOK,
            arg_1: None,
            arg_2: None,
            arg_3: None,
        }
    }
}

fn get_scene<'a>(components: &'a data::Components, state: & State) -> Scene<'a> {
    let entity_ids = components.locations.get(&state.current_location).unwrap();
    let mut scene: Scene<'a> = Scene{
        location_id: state.current_location,
        entity_ids: entity_ids,
        exit_ids: entity_ids.
            into_iter().
            filter(|&id| components.destinations.contains_key(id)).
            collect(),
        actions: Vec::new(),
    };
    // this is a waste of memory
    scene.actions.push(Action{action_type: ActionType::LOOK, ..Default::default()});
    for exit_id in &scene.exit_ids {
        scene.actions.push(Action{
            action_type: ActionType::GO, 
            arg_1: Some(**exit_id), 
            ..Default::default()
        });
    }

    return scene;
}

fn render(scene: & Scene, _state: & State, components: & data::Components) {
    println!("You are at {}", components.names.get(&scene.location_id).unwrap());
    println!("You see {}", components.descriptions.get(&scene.location_id).unwrap());
    for entity_id in scene.entity_ids {
        println!("{}", components.names.get(entity_id).unwrap());
    }
    let mut action_id: usize = 1;
    for action in scene.actions.iter() {
        println!("{}. {}", action_id, render_action(& action, components));
        action_id = action_id + 1;
    }
}

fn render_detailed(scene: & Scene, _state: & State, components: & data::Components) {
    println!("You see {}", components.descriptions.get(&scene.location_id).unwrap());
    for entity_id in scene.entity_ids {
        println!("{}", components.descriptions.get(entity_id).unwrap());
    }
}

fn render_action(action: & Action, components: & data::Components) -> String {
    return match action.action_type {
        ActionType::GO => {
            return format!("Go to {}", components.names.get(&action.arg_1.unwrap()).unwrap());
        }
        ActionType::LOOK => String::from("Look around"),
        ActionType::TALK => {
            return format!("Speak to {}", components.names.get(&action.arg_1.unwrap()).unwrap());
        }
    }
}
