#[warn(non_shorthand_field_patterns)]
// std
use std::collections::HashSet;

// int
use super::action::{Action};
use super::constants::{key_to_char, NUMBERS, TYPEABLE};
use super::conversation_action::ConversationAction;
use super::game::Game;
use super::game_action::GameAction;
use super::interface::Interface;
use super::main_menu_action::{MainMenuAction};
use super::mode::{Mode};
use macroquad::prelude::{
	KeyCode,
	get_char_pressed,
	get_keys_released,
};


impl Interface {
	pub fn check_input_load(&mut self) -> Option<isize> {
		let key_set = get_keys_released();
		if key_set.contains(&KeyCode::Escape) {
			return Some(-1);
		} else if key_set.contains(&KeyCode::Enter) {
			match self.input_buffer.parse::<isize>() {
				Ok(index) => { return Some(index) },
				Err(_) => { self.error(&Mode::LOAD, "Bad file index 2"); },
			}
		} else {
			let diff: HashSet<_> = key_set.intersection(&self.numbers).collect();
			for key in diff {
				match NUMBERS.iter().position(|&r| r == *key) {
					Some(index) => {
						self.input_buffer.push_str(format!("{}", index).as_str());
					},
					None => {}
				}
			}
		}

		return None;
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
		let key_set = get_keys_released();

		if key_set.contains(&KeyCode::Q) {
			// add some 'game not saved' check
			// or put a menu to save
			// maybe this should say: go to main menu
			return Err(GameAction::QUIT);
		} else if key_set.contains(&KeyCode::S) {
			return Err(GameAction::SAVE);
		} else {
			let diff: HashSet<&KeyCode> = key_set.intersection(&self.numbers).collect();
			for key in diff.iter() {
				match TYPEABLE.iter().position(|&r| r == **key) {
					Some(pos) => {
						return Ok(Some(game.scene.actions[pos - 1]));
					},
					None => {}
				}
			}
		}

		return Ok(None);
	}

	pub fn check_input_save(&mut self) -> Option<String> {
		let key_set = get_keys_released();

		if key_set.contains(&KeyCode::Enter) {
			return Some(self.input_buffer.clone());
		} else if key_set.contains(&KeyCode::Backspace) {
			self.input_buffer.pop();
		} else {
			let diff: HashSet<&KeyCode> = key_set.intersection(&self.typeable).collect();
			for key in diff.iter() {
				match TYPEABLE.iter().position(|&r| r == **key) {
					Some(_) => {
						self.input_buffer.push_str(format!("{}", key_to_char(**key as KeyCode)).as_str());
					},
					None => {}
				}
			}
		}

		return None;
	}

	pub fn check_input_talk(&mut self, game: &Game) -> Result<ConversationAction, GameAction> {
		let key_set = get_keys_released();

		if key_set.contains(&KeyCode::Q) {
			// add some 'game not saved' check
			// or put a menu to save
			// maybe this should say: go to main menu
			return Err(GameAction::QUIT);
		} else if key_set.contains(&KeyCode::S) {
			return Err(GameAction::SAVE);
		} else if key_set.contains(&KeyCode::B) {
			return Ok(ConversationAction::BACK);
		} else if key_set.contains(&KeyCode::E) {
			return Ok(ConversationAction::END);
		} else {
			let diff: HashSet<&KeyCode> = key_set.intersection(&self.numbers).collect();
			// check against number of available options
			for key in diff.iter() {
				match TYPEABLE.iter().position(|&r| r == **key) {
					Some(pos) => {
						return Ok(ConversationAction::ADD(pos - 1));
					},
					None => {}
				}
			}
		}

		return Ok(ConversationAction::NONE);
	}
}
