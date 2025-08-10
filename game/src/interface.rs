#[warn(non_shorthand_field_patterns)]
// std
use std::collections::HashSet;
use std::collections::VecDeque;

// ext
use macroquad::prelude::{Color, KeyCode};

// int
use super::mode::{Mode};
use super::constants::{NUMBERS, TYPEABLE};

pub struct Theme {
	pub input_background: Color,
}

pub struct Interface {
	pub input_buffer: String,
	pub text: VecDeque<String>,
	pub numbers: HashSet<KeyCode>,
	pub theme: Theme,
	pub typeable: HashSet<KeyCode>,
}

impl Interface {
	pub fn new() -> Interface {
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

	// clear...something. Need better name
	pub fn clear(&mut self) {
		while self.text.len() > 0 {
			let _ = self.text.pop_front();
		}
	}

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

	pub fn println(&mut self, s: String) {
		self.text.push_back(s);
		while self.text.len() > 30 {
			let _ = self.text.pop_front();
		}
	}

	pub fn println_str(&mut self, s: &str) {
		self.println(String::from(s));
	}
}
