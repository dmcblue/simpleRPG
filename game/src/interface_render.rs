#[warn(non_shorthand_field_patterns)]
// ext
use macroquad::prelude::{
	BLACK,
	BLUE,
	GREEN,
	clear_background,
	draw_rectangle,
	draw_text,
	screen_height,
	screen_width,
};

// int
use super::action::{Action, ActionType};
use super::game::Game;
use super::interface::Interface;
use super::data::{Components, ConversationNode, Price, Vending};

impl Interface {
	pub fn render_action(&self, game: &Game, action: &Action) -> String {
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
			ActionType::VEND => {
				return format!("Buy/Sell with {}", game.components.names[action.arg_1.unwrap()]);
			}
		}
	}

	pub fn render_action_taken(&mut self, game: &Game, action: &Action) {
		match action.action_type {
			ActionType::CHECK_INVENTORY => {
				self.println_str("In your inventory:");
				let entity_ids: Vec<usize> = game.components.locations[game.components.inventory_id].to_vec();
				if entity_ids.len() == 0 {
					self.println_str("Nothing");
				}
				for entity_id in entity_ids {
					self.println(format!("{}", game.components.names[entity_id]));
				}
			}
			ActionType::GO => {
				self.println(format!("You go to {}", game.components.names[game.scene.location_id]));
				self.println(format!("You see {}", game.components.descriptions[game.scene.location_id]));
				for entity_id in &game.scene.entity_ids {
					self.println(format!("{}", game.components.names[*entity_id]));
				}
			}
			ActionType::LOOK => self.render_location_detailed(game),
			ActionType::TAKE => {
				self.println(format!("You put {} in your inventory", game.components.names[action.arg_1.unwrap()]));
			}
			ActionType::TALK => {
				self.println(format!("You turn to {} and say:", game.components.names[action.arg_1.unwrap()]));
			}
			ActionType::VEND => {
				self.println(format!("You haggle with {}:", game.components.names[action.arg_1.unwrap()]));
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

	pub fn render_conversation(&mut self, conversation_node: &ConversationNode) {
		let mut i = 1;
		for prompt in &conversation_node.prompts {
			if prompt.enabled {
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
		self.println(format!("You see {}", game.components.descriptions[game.scene.location_id]));
		for entity_id in &game.scene.entity_ids {
			self.println(format!("{}", game.components.descriptions[*entity_id]));
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

	pub fn render_price(&self, price: &Price) -> String {
		match price {
			Price::Range(min, max) => {
				// @TODO how to do random in rust
				return format!("{} gold", min);
			}
		}
	}

	pub fn render_vending(&mut self, vending: &Vending, components: &Components) {
		let mut i = 1;
		for vend_item in &vending.items {
			components.names[components.get_array_id(&vend_item.id)];
			self.println(
				format!(
					"{}. {} - {}",
					i,
					components.names[components.get_array_id(&vend_item.id)], //uuid?
					self.render_price(&vend_item.price)
				)
			);

			i = i + 1;
		}

		self.println_str("(B)ack");
	}

	fn render(frame: &Frame) {
		clear_background(BLACK);

		let mut i = 0.0;
		for line in frame._frame.iter() {
			let t: String = line.iter().collect();
			draw_text(t.as_str(), 10.0, 20.0 * (i + 1.0), 18.0, GREEN);
			
			i = i + 1.0;
		}
	}

	// this could be a view template
	// that takes the interface or something
	pub fn render_load(&mut self) {
		// frame.write(109, 39, "X");
		// clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			// draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			frame.text(0, i, line);
			i = i + 1.0;
		}

		// DAN LEFT OFF
		draw_rectangle(0.0, screen_height() - 30.0, screen_width(), screen_height(), self.theme.input_background);
		draw_text(self.input_buffer.as_str(), 10.0, screen_height() - 15.0, 20.0, BLACK);
	}

	pub fn render_main_menu(&mut self) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}
	}

	pub fn render_play(&mut self) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}


		draw_text("(q)uit | (s)ave", 10.0, screen_height() - 20.0, 18.0, BLUE);
	}

	pub fn render_save(&mut self) {
		clear_background(BLACK);
		draw_text("Name your save:", 10.0, 15.0, 20.0, GREEN);
		draw_rectangle(0.0, screen_height() - 30.0, screen_width(), screen_height(), self.theme.input_background);
		draw_text(self.input_buffer.as_str(), 10.0, screen_height() - 15.0, 20.0, BLACK);
	}
}
