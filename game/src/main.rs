use std::io::{self, Write};

mod data;

#[derive(PartialEq, Clone)]
enum ActionType {
	CHECK_INVENTORY,
	GO,
	LOOK,
	TAKE,
	TALK
}
// #[derive(Default)]
struct Action {
	action_type: ActionType,
	arg_1: Option<usize>,
	// arg_2: Option<usize>,
	// arg_3: Option<usize>,
}
// #[derive(Default)]
struct Scene {
	location_id: usize,
	entity_ids: Vec<usize>,
	exit_ids: Vec<usize>,
	takeable_item_ids: Vec<usize>,
	actions: Vec<Action>,
}
impl Default for Action {
	fn default() -> Action {
		Action {
			action_type: ActionType::LOOK,
			arg_1: None,
			// arg_2: None,
			// arg_3: None,
		}
	}
}

struct State {
	pub current_location: usize,
	pub last_action_type: ActionType,
}

struct Game<'a> {
	state: State,
	components: data::Components<'a>,
	scene: Scene
}

impl Game<'_> {
	fn setup_scene(&mut self) {
		let entity_ids: Vec<usize> = self.components.locations[self.state.current_location].to_vec();
		self.scene.location_id = self.state.current_location;
		self.scene.entity_ids = entity_ids.clone();
		self.scene.exit_ids = entity_ids.clone(). //performance
				into_iter().
				filter(|id| {
					if *id < self.components.exits_start {
						return false;
					} 
					self.components.destinations.contains(&(*id - self.components.exits_start))
				}).
				collect();
		self.scene.takeable_item_ids = entity_ids.clone(). //performance
				into_iter().
				filter(|id| {
					if *id < self.components.items_start || *id >= self.components.people_start {
						return false;
					} 
					self.components.takeable[*id - self.components.items_start]
				}).
				collect();
		self.scene.actions = Vec::new();

		// this is a waste of memory
		self.scene.actions.push(Action{action_type: ActionType::LOOK, ..Default::default()});
		for exit_id in &self.scene.exit_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::GO, 
					arg_1: Some(*exit_id), 
					..Default::default()
				}
			);
		}
		for item_id in &self.scene.takeable_item_ids {
			self.scene.actions.push(
				Action{
					action_type: ActionType::TAKE, 
					arg_1: Some(*item_id), 
					..Default::default()
				}
			);
		}
		self.scene.actions.push(Action{action_type: ActionType::CHECK_INVENTORY, ..Default::default()});
	}

	fn render(&self) {
		if self.state.last_action_type == ActionType::GO {
			println!("You are at {}", self.components.names[self.scene.location_id]);
			println!("You see {}", self.components.descriptions[self.scene.location_id]);
			for entity_id in &self.scene.entity_ids {
				println!("{}", self.components.names[*entity_id]);
			}
		}
		let mut action_id: usize = 1;
		for action in self.scene.actions.iter() {
			println!("{}. {}", action_id, self.render_action(& action));
			action_id = action_id + 1;
		}
	}

	fn render_detailed(&self) {
		println!("You see {}", self.components.descriptions[self.scene.location_id]);
		for entity_id in &self.scene.entity_ids {
			println!("{}", self.components.descriptions[*entity_id]);
		}
	}

	fn render_action(&self, action: &Action) -> String {
		return match action.action_type {
			ActionType::CHECK_INVENTORY => {
				return String::from("Check your inventory");
			}
			ActionType::GO => {
				return format!("Go to {}", self.components.names[action.arg_1.unwrap()]);
			}
			ActionType::LOOK => String::from("Look around"),
			ActionType::TAKE => {
				return format!("Take {}", self.components.names[action.arg_1.unwrap()]);
			}
			ActionType::TALK => {
				return format!("Speak to {}", self.components.names[action.arg_1.unwrap()]);
			}
		}
	}

	fn render_inventory(&self) {
		println!("In your inventory:");
		let entity_ids: Vec<usize> = self.components.locations[self.components.inventory_id].to_vec();
		for entity_id in entity_ids {
			println!("{}", self.components.names[entity_id]);
		}
	}

	fn update<'a>(&mut self, input: String) -> Result<(), &'a str> {
		if input == "quit" {
			return Err("Goodbye!");
		} else {
			let mut i: usize = input.parse::<usize>().unwrap();
			i = i - 1;
			let action = &self.scene.actions[i];
			self.state.last_action_type = action.action_type.clone();
			match action.action_type {
				ActionType::CHECK_INVENTORY => self.render_inventory(),
				ActionType::GO => {
					self.state.current_location = 
						self.components.destinations[action.arg_1.unwrap() - self.components.exits_start]
				},
				ActionType::LOOK => self.render_detailed(),
				ActionType::TAKE => {
					let id = action.arg_1.unwrap();
					let index = self.components.locations[self.state.current_location].iter().position(|eid| *eid == id).unwrap();
					self.components.locations[self.state.current_location].remove(index);
					self.components.locations[self.components.inventory_id].push(id);
					// record change
				},
				ActionType::TALK => ()
				// _ => ()
			}
		}
		return Ok(());
	}
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

	let mut game = Game {
		components: data::make_components(),
		scene: Scene{
			location_id: 0,
			entity_ids: Vec::new(),
			exit_ids: Vec::new(),
			takeable_item_ids: Vec::new(),
			actions: Vec::new(),
		},
		state: State {
			current_location: data::main::get_start_location_id(),
			last_action_type: ActionType::GO,
		},
	};
	// let mut components = data::make_components();

	// let mut state = State {
	// 	current_location: data::main::get_start_location_id(),
	// };
	data::main::load_data(&mut game.components);

	loop {
		game.setup_scene();
		game.render();
		let input: String = get_input("> ");
		match game.update(input) {
			Err(s) => { println!("{}", s); break; },
			_ => ()
		}
	}
}
