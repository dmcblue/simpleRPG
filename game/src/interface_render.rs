#[warn(non_shorthand_field_patterns)]
// std

// ext
use rand::prelude::*;

// int
use super::action::{Action, ActionType};
use super::game::Game;
use super::interface::Interface;
use super::data::{
	Components,
	ConversationNode,
	Vending
};
use super::renderer::{
	// Frame,
	Renderer,
	// MacroquadRenderer
};

impl Interface {
	pub fn render_action(&self, game: &Game, action: &Action) -> String {
		return match action.action_type {
			ActionType::CHALLENGE => {
				return format!("Challenge: {}", game.components.get_name(action.arg_1.unwrap()));
			}
			ActionType::CHECK_INVENTORY => {
				return String::from("Check your inventory");
			}
			ActionType::GO => {
				return format!("Go to {}", game.components.get_name(action.arg_1.unwrap()));
			}
			ActionType::LOOK => String::from("Look around"),
			ActionType::TAKE => {
				// return format!("Take {} {}s", action.arg_2.unwrap(), game.components.names[action.arg_1.unwrap()]);
				return format!("Take {} {}s", action.arg_2.unwrap(), game.components.get_name(action.arg_1.unwrap()));
			}
			ActionType::TALK => {
				return format!("Speak to {}", game.components.get_name(action.arg_1.unwrap()));
			}
			ActionType::VEND => {
				return format!("Buy/Sell with {}", game.components.get_name(action.arg_1.unwrap()));
			}
		}
	}

	pub fn render_action_taken(&mut self, game: &Game, action: &Action) {
		match action.action_type {
			// @DAN TODO
			ActionType::CHALLENGE => {
				self.println(format!("You take the challenge '{}'", game.components.get_name(action.arg_1.unwrap())));
			}
			ActionType::CHECK_INVENTORY => {
				let mut any = false;
				self.println_str("In your inventory:");
				// for (entity_uuid, quantity) in game.components.location_items[game.components.inventory_id].iter() {
				for (entity_uuid, quantity) in game.components.read_location_items(game.components.inventory_uuid).iter() {
					if *quantity > 0 {
						any = true;
						let name = game.components.get_name(*entity_uuid);
						self.println(format!(
							" - ({}) {}",
							*quantity,
							// game.components.names[game.components.get_array_id(entity_uuid)]
							name
						));
					}
				}

				if !any {
					self.println_str("Nothing");
				}
			}
			ActionType::GO => {
				// self.println(format!("You go to {}", game.components.names[game.scene.location_id]));
				self.println(format!("You go to {}", game.components.get_name(game.scene.location_uuid)));
				// self.println(format!("You see {}", game.components.descriptions[game.scene.location_id]));
				self.println(format!("You see {}", game.components.get_description(game.scene.location_uuid)));
				for entity_uuid in &game.scene.entity_uuids {
					// self.println(format!("{}", game.components.names[*entity_id]));
					self.println(format!("{}", game.components.get_name(*entity_uuid)));
				}
			}
			ActionType::LOOK => self.render_location_detailed(game),
			ActionType::TAKE => {
				self.println(format!(
					"You put {} {}s in your inventory",
					action.arg_2.unwrap(),
					// game.components.names[action.arg_1.unwrap()],
					game.components.get_name(action.arg_1.unwrap()),
				));
			}
			ActionType::TALK => {
				self.println(format!("You turn to {} and say:", game.components.get_name(action.arg_1.unwrap())));
			}
			ActionType::VEND => {
				self.println(format!("You haggle with {}:", game.components.get_name(action.arg_1.unwrap())));
			}
		}
	}

	pub fn render_actions(&mut self, game: &Game) {
		let mut action_id: usize = 1;
		for action in game.scene.actions.iter() {
			self.println(format!("{}. {}", action_id, self.render_action(&game, &action)));
			action_id = action_id + 1;
		}
	}

	pub fn render_conversation(&mut self, game: &Game, conversation_node: &ConversationNode) {
		let mut i = 1;
		for prompt in &conversation_node.prompts {
			// if *game.components.enabled.get(&prompt.id).unwrap() {
			if game.components.is_enabled(prompt.uuid) {
				self.println(format!("{}. {}", i, prompt.prompt));

				i = i + 1;
			}
		}

		self.println_str("(B)ack");
		self.println_str("(E)nd");
	}

	pub fn render_conversation_response(&mut self, response: &String) {
		self.println_str(response.as_str());
	}

	pub fn render_location_detailed(&mut self, game: &Game) {
		self.println(format!("You see {}", game.components.get_description(game.scene.location_uuid)));
		for entity_uuid in &game.scene.entity_uuids {
			self.println(format!("{}", game.components.get_description(*entity_uuid)));
		}
	}

	pub fn render_save_files(&mut self, file_names: Vec<String>) {
		let mut i: usize = 1;
		for file_name in file_names {
			self.println(format!("{}. {}", i, file_name));
			i = i + 1;
		}
		self.println(String::from("(esc) to cancel"));
	}

	pub fn render_saved(&mut self) {
		self.println(String::from("Game saved."));
	}

	pub fn render_vending(&mut self, vending: &Vending, components: &Components) {
		let mut i = 1;
		for vend_item in &vending.items {
			// components.names[components.get_array_id(&vend_item.id)];
			self.println(
				format!(
					"{}. {} - {} {}",
					i,
					components.get_name(vend_item.uuid), //uuid?
					&vend_item.price.quantity,
					components.get_name(vend_item.price.item_uuid),
				)
			);

			i = i + 1;
		}

		self.println_str("(B)ack");
	}

	fn render(&mut self) {
		self.renderer.render(&mut self.frame);
	}

	pub fn render_input(&mut self) {
		self.frame.rect(0,self.frame.height - 3,self.frame.width, 3, '#');
		self.frame.text(1,self.frame.height - 2, self.input_buffer.as_str());
	}

	pub fn render_global_menu(&mut self) {
		// this hard code is a mystery
		self.frame.text(0,self.frame.height - 3, "(q)uit | (s)ave");
	}

	pub fn render_hr(&mut self){
		self.println_str("    ---- ---- ----");
	}

	pub fn render_log(&mut self) {
		let mut i: usize = 0;
		for line in self.text.iter() {
			self.frame.clear_line(i);
			self.frame.text(0, i, line.as_str());
			i = i + 1;
		}
	}

	// this could be a view template
	// that takes the interface or something
	pub fn render_load(&mut self) {
		self.render_log();
		self.render_input();
		self.render();
	}

	pub fn render_main_menu(&mut self) {
		self.render_log();
		self.render();
	}

	pub fn render_play(&mut self) {
		self.render_log();
		self.render_global_menu();
		self.render();
	}

	pub fn render_save(&mut self) {
		self.frame.text(1, 0, "Name your save:");
		self.render_input();
		self.render();
	}
}
