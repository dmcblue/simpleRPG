#[warn(non_shorthand_field_patterns)]
use std::collections::HashSet;
use std::mem::ManuallyDrop;
use super::action::{Action, ActionType};
use super::game::Game;
use macroquad::{
	prelude::*,
};
use std::collections::VecDeque;
use super::game_action::GameAction;
use super::mode::{Mode};
use super::main_menu_action::{MainMenuAction};
use super::constants::{key_to_char, NUMBERS, TYPEABLE};

struct Theme {
	input_background: Color,
}

pub struct MacroquadInterface {
	input_buffer: String,
	text: VecDeque<String>,
	numbers: HashSet<KeyCode>,
	theme: Theme,
	typeable: HashSet<KeyCode>,
}

impl MacroquadInterface {
	pub fn new() -> MacroquadInterface {
		Self {
			input_buffer: String::new(),
			text: VecDeque::new(),
			numbers: HashSet::from(NUMBERS),
			theme: Theme{
				input_background: Color::new(0.8, 0.8, 0.8, 1.00),
			},
			typeable: HashSet::from(TYPEABLE),
		}
	}

	// private

	// clear...something. Need better name
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
			Mode::LOAD => {
				self.input_buffer = String::new();
			},
			Mode::MAIN_MENU => {
				self.println_str("(N)ew Game");
				self.println_str("(L)oad Game");
				self.println_str("(Q)uit");
			},
			Mode::PLAY => {},
			Mode::SAVE => {
				self.input_buffer = String::new();
			}
		}
	}

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

	pub fn error(&mut self, mode: &Mode, err_str: &str) {
		println!("{}", err_str);
		match mode {
			Mode::LOAD => {
				self.input_buffer = String::new();
			},
			Mode::MAIN_MENU => {},
			Mode::PLAY => {},
			Mode::SAVE => {},
		}
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

	pub fn render_load(&mut self) {
		clear_background(BLACK);

		let mut i: f32 = 0.0;
		for line in self.text.iter() {
			draw_text(line, 10.0, 20.0 + (20.0 * i), 18.0, GREEN);
			i = i + 1.0;
		}

		draw_rectangle(0.0, screen_height() - 30.0, screen_width(), screen_height(), self.theme.input_background);
		draw_text(self.input_buffer.as_str(), 10.0, screen_height() - 15.0, 20.0, BLACK);
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
}
