use std::io::{self, Write};
use super::action::{Action, ActionType};
use super::game::Game;
use macroquad::{
	prelude::*,
	ui::{hash, root_ui, widgets::InputText}
};
use std::collections::VecDeque;
use super::game_action::GameAction;
use super::mode::Mode;
use super::main_menu_action::MainMenuAction;

pub struct MacroquadInterface {
	pub text: VecDeque<String>
}

impl MacroquadInterface {
	// private

	// clear...something. Need name
	fn clear(&mut self) {
		while self.text.len() > 0 {
			let _ = self.text.pop_front();
		}
	}

	fn println(&mut self, s: String) {
		self.text.push_back(s);
		while self.text.len() > 30 {
			let _ = self.text.pop_front();
		}
	}

	fn println_str(&mut self, s: &str) {
		self.println(String::from(s));
	}

	// public
	pub fn change_mode(&mut self, mode: &Mode) {
		self.clear();
		match *mode {
			Mode::MAIN_MENU => {
				self.println_str("(N)ew Game");
				self.println_str("(L)oad Game");
				self.println_str("(Q)uit");
			},
			Mode::PLAY => {}
		}
	}

	pub fn check_input_main_menu(&self) -> Option<MainMenuAction> {
		if let Some(ch) = get_char_pressed() {
			match ch {
				'n' => { return Some(MainMenuAction::NEW_GAME); },
				'l' => { return Some(MainMenuAction::LOAD_GAME); },
				'q' => { return Some(MainMenuAction::QUIT); },
				_ => ()
			}
		}

		return None;
	}

	pub fn check_input_play(&self, game: &Game) -> Result<Option<Action>, GameAction> {
		if let Some(ch) = get_char_pressed() {
			if ch == 'q' {
				// add some 'game not saved' check
				// or put a menu to save
				// maybe this should say: go to main menu
				return Err(GameAction::QUIT);
			} else if ch == 's' {
				return Err(GameAction::SAVE);
			} else {
				let i: usize = <usize as TryInto<usize>>::try_into((ch as usize)).unwrap() - 1;
				if i > 47 && i - 48 < game.scene.actions.len() {
					return Ok(Some(game.scene.actions[i - 48]));
				}
			}
		}

		return Ok(None);
	}

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
		}
	}

	pub fn render_actions(&mut self, game: &Game) {
		let mut action_id: usize = 1;
		for action in game.scene.actions.iter() {
			self.println(format!("{}. {}", action_id, self.render_action(&game, &action)));
			action_id = action_id + 1;
		}
	}

	pub fn render_location_detailed(&mut self, game: &Game) {
		self.println(format!("You see {}", game.components.descriptions[game.scene.location_id]));
		for entity_id in &game.scene.entity_ids {
			self.println(format!("{}", game.components.descriptions[*entity_id]));
		}
	}

	pub fn render_main_menu(&mut self) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}
	}

	pub fn render_save(&mut self) {
		self.println(String::from("Game saved."))
	}

	pub fn render_play(&mut self, game: &Game) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}


		draw_text("(q)uit | (s)ave", 10.0, screen_height() - 20.0, 18.0, BLUE);
	}
}
