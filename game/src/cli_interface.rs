use std::io::{self, Write};
use super::action::{Action, ActionType};
use super::interface::Interface;
use super::game::Game;

pub struct CliInterface {}

impl Interface for CliInterface {
	fn get_input(&self) -> String{
		let mut stdout = io::stdout();
		let _ = stdout.write_all("> ".as_bytes());
		let _ = stdout.flush();
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_goes_into_input_above) => {},
			Err(_no_updates_is_fine) => {},
		}
		return input.trim().to_string();
	}

	fn render(&self, game: &Game) {
		if game.state.last_action_type == ActionType::GO {
			println!("You are at {}", game.components.names[game.scene.location_id]);
			println!("You see {}", game.components.descriptions[game.scene.location_id]);
			for entity_id in &game.scene.entity_ids {
				println!("{}", game.components.names[*entity_id]);
			}
		}
		let mut action_id: usize = 1;
		for action in game.scene.actions.iter() {
			println!("{}. {}", action_id, self.render_action(&game, &action));
			action_id = action_id + 1;
		}
	}

	fn render_detailed(&self, game: &Game) {
		println!("You see {}", game.components.descriptions[game.scene.location_id]);
		for entity_id in &game.scene.entity_ids {
			println!("{}", game.components.descriptions[*entity_id]);
		}
	}

	fn render_action(&self, game: &Game, action: &Action) -> String {
		return match action.action_type {
			ActionType::CHECK_INVENTORY => {
				return String::from("Check your inventory");
			}
			ActionType::GO => {
				return format!("Go to {}", game.components.names[action.arg_1.unwrap()]);
			}
			ActionType::LOOK => String::from("Look around"),
			ActionType::TAKE => {
				return format!("Take {}", game.components.names[action.arg_1.unwrap()]);
			}
			ActionType::TALK => {
				return format!("Speak to {}", game.components.names[action.arg_1.unwrap()]);
			}
		}
	}

	fn render_inventory(&self, game: &Game) {
		println!("In your inventory:");
		let entity_ids: Vec<usize> = game.components.locations[game.components.inventory_id].to_vec();
		for entity_id in entity_ids {
			println!("{}", game.components.names[entity_id]);
		}
	}
}
