use std::io::{self, Write};
use super::action::{Action, ActionType};
use super::interface::Interface;
use super::game::Game;
use macroquad::{
	prelude::*,
	ui::{hash, root_ui, widgets::InputText}
};
use std::collections::VecDeque;

pub struct MacroquadInterface {
	pub text: VecDeque<String>
}

impl MacroquadInterface {
	fn println(&mut self, s: String) {
		self.text.push_back(s);
		while self.text.len() > 30 {
			let _ = self.text.pop_front();
		}
	}

	fn draw_editor(&self) {
		let mut data = String::new();

		let window_id = hash!();
		root_ui().window(
			window_id,
			vec2(0.0, 200.0),
			vec2(screen_width(), 230.0),
			|ui| {
				let input_text_id = hash!();
				InputText::new(input_text_id)
					.label("")
					.size(vec2(screen_width() - 4.0, 30.0 - 4.0))
					.ui(ui, &mut data);
			},
		);
	}

	pub fn check_input(&self, game: &Game) -> Result<Option<Action>, &str> {
		if let Some(ch) = get_char_pressed() {
			if ch == 'q' {
				return Err("quit");
			} else {
				let i: usize = <usize as TryInto<usize>>::try_into((ch as usize)).unwrap() - 1;
				if i > 47 && i - 48 < game.scene.actions.len() {
					return Ok(Some(game.scene.actions[i - 48]));
				}
			}
		}

		return Ok(None);
	}

	pub fn render_action_taken(&mut self, game: &Game, action: &Action) {
		match action.action_type {
			ActionType::CHECK_INVENTORY => {
				self.println(String::from("In your inventory:"));
				let entity_ids: Vec<usize> = game.components.locations[game.components.inventory_id].to_vec();
				if entity_ids.len() == 0 {
					self.println(String::from("Nothing"));
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
			ActionType::LOOK => self.render_detailed(game),
			ActionType::TAKE => {
				self.println(format!("You put {} in your inventory", game.components.names[action.arg_1.unwrap()]));
			}
			ActionType::TALK => {
				self.println(format!("You turn to {} and say:", game.components.names[action.arg_1.unwrap()]));
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
}

impl Interface for MacroquadInterface {
	fn init(&self) {
		clear_background(BLACK);
	}

	fn get_input(&self) -> String{
		return String::new();
	}

	fn render(&mut self, game: &Game) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line.as_str(), 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}

		
		draw_text("(q)uit", 10.0, screen_height() - 20.0, 18.0, BLUE);
	}

	fn render_detailed(&mut self, game: &Game) {
		self.println(format!("You see {}", game.components.descriptions[game.scene.location_id]));
		for entity_id in &game.scene.entity_ids {
			self.println(format!("{}", game.components.descriptions[*entity_id]));
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

	fn open_inventory(&mut self, game: &Game) {
		
	}
}
