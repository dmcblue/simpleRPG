#[warn(non_shorthand_field_patterns)]
// std
use std::collections::{HashSet, VecDeque};

// ext
use macroquad::prelude::{Color};

// int
use super::input::{
	// MacroquadInput,
	RatatuiInput,
	KeyCode,
	NUMBERS,
	TYPEABLE
};
use super::mode::{Mode};
use super::renderer::{
	Frame,
	// Renderer,
	// MacroquadRenderer,
	RatatuiRenderer
};

const TEXT_LENGTH: usize = 20;

pub struct Theme {
	pub input_background: Color,
}

pub struct Interface {
	pub frame: Frame,
	pub input: RatatuiInput,
	pub input_buffer: String,
	pub numbers: HashSet<KeyCode>,
	pub renderer: RatatuiRenderer,
	// pub theme: Theme,
	pub text: VecDeque<String>, // log
	pub typeable: HashSet<KeyCode>,
}

impl Interface {
	pub fn new() -> Interface {
		let mut s = Self {
			frame: Frame::new(),
			input: RatatuiInput::new(),
			input_buffer: String::new(),
			text: VecDeque::with_capacity(TEXT_LENGTH),
			numbers: HashSet::from(NUMBERS),
			// renderer: MacroquadRenderer::new(),
			renderer: RatatuiRenderer::new(),
			// theme: Theme{
			// 	input_background: Color::new(0.8, 0.8, 0.8, 1.00),
			// },
			typeable: HashSet::from(TYPEABLE),
		};
		// trying to start the text at the bottom of the log area but failing
		for _n in 0..30 {
			s.println_str("");
		}
		s
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
			// Mode::CONVERSATION => {},
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
			// Mode::CONVERSATION => {},
			Mode::LOAD => {
				self.input_buffer = String::new();
			},
			Mode::MAIN_MENU => {},
			Mode::PLAY => {},
			Mode::SAVE => {},
		}
	}

	pub fn println(&mut self, line: String) {
		let sub_lines: Vec<String> = self.frame.split_line(0, line.as_str());
		for sub_line in sub_lines {
			self.text.push_back(sub_line);
		}
		// self.text.push_back(s);
		while self.text.len() > TEXT_LENGTH {
			let _ = self.text.pop_front();
		}
	}

	pub fn println_str(&mut self, s: &str) {
		self.println(String::from(s));
	}
}
