// std
use std::vec::Vec;
use std::collections::{HashSet};

use ratatui::crossterm::event::{
	self,
	Event,
	KeyCode as CtKeyCode,
	KeyEvent,
	KeyEventKind,
	KeyModifiers
};

use super::key_code::{
	KeyCode,
};
use super::input::Input;

pub struct RatatuiInput {
	buffer: Vec<KeyCode>,
}

impl RatatuiInput {
	pub fn new() -> Self {
		Self{
			buffer: Vec::new(),
		}
	}

	fn convert(&self, key_code: CtKeyCode) -> KeyCode {
		return match key_code {
			CtKeyCode::Backspace => KeyCode::Backspace,
			CtKeyCode::Enter => KeyCode::Enter,
			CtKeyCode::Esc => KeyCode::Escape,
			CtKeyCode::Char(' ') => KeyCode::Space,
			CtKeyCode::Char('0') => KeyCode::Key0,
			CtKeyCode::Char('1') => KeyCode::Key1,
			CtKeyCode::Char('2') => KeyCode::Key2,
			CtKeyCode::Char('3') => KeyCode::Key3,
			CtKeyCode::Char('4') => KeyCode::Key4,
			CtKeyCode::Char('5') => KeyCode::Key5,
			CtKeyCode::Char('6') => KeyCode::Key6,
			CtKeyCode::Char('7') => KeyCode::Key7,
			CtKeyCode::Char('8') => KeyCode::Key8,
			CtKeyCode::Char('9') => KeyCode::Key9,
			CtKeyCode::Char('a') => KeyCode::A,
			CtKeyCode::Char('b') => KeyCode::B,
			CtKeyCode::Char('c') => KeyCode::C,
			CtKeyCode::Char('d') => KeyCode::D,
			CtKeyCode::Char('e') => KeyCode::E,
			CtKeyCode::Char('f') => KeyCode::F,
			CtKeyCode::Char('g') => KeyCode::G,
			CtKeyCode::Char('h') => KeyCode::H,
			CtKeyCode::Char('i') => KeyCode::I,
			CtKeyCode::Char('j') => KeyCode::J,
			CtKeyCode::Char('k') => KeyCode::K,
			CtKeyCode::Char('l') => KeyCode::L,
			CtKeyCode::Char('m') => KeyCode::M,
			CtKeyCode::Char('n') => KeyCode::N,
			CtKeyCode::Char('o') => KeyCode::O,
			CtKeyCode::Char('p') => KeyCode::P,
			CtKeyCode::Char('q') => KeyCode::Q,
			CtKeyCode::Char('r') => KeyCode::R,
			CtKeyCode::Char('s') => KeyCode::S,
			CtKeyCode::Char('t') => KeyCode::T,
			CtKeyCode::Char('u') => KeyCode::U,
			CtKeyCode::Char('v') => KeyCode::V,
			CtKeyCode::Char('w') => KeyCode::W,
			CtKeyCode::Char('x') => KeyCode::X,
			CtKeyCode::Char('y') => KeyCode::Y,
			CtKeyCode::Char('z') => KeyCode::Z,
			_ => KeyCode::Space,
		}
	}

	fn convert_modifier(&self, key_code: KeyModifiers) -> KeyCode {
		return match key_code {
			KeyModifiers::CONTROL => KeyCode::Control,
			_ => KeyCode::Control, // this makes no sense but whatever
		}
	}

	// from crossterm example
	/// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
		for k in key.modifiers {
			self.buffer.push(self.convert_modifier(k));
		}
		self.buffer.push(self.convert(key.code));
    }
}

impl Input for RatatuiInput {
	fn get_keys_released(&mut self) -> HashSet<KeyCode> {
		let keyset: HashSet<KeyCode> = self.buffer.iter().map(|k| *k ).collect();
		self.buffer.clear();
		return keyset;
	}

	fn get_key_pressed(&mut self) -> Option<KeyCode> {
		if self.buffer.is_empty() {
			return None;
		}

		let key: KeyCode = self.buffer.pop()?;
		self.buffer.clear();
		return Some(key);
	}

	/// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
	fn update(&mut self) {
		match event::read() {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            // Event::Mouse(_) => {}
            // Event::Resize(_, _) => {}
            _ => {}
        }
	}
}
