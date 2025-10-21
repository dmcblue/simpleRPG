// std
use std::collections::HashSet;

use macroquad::prelude::{
	get_keys_released as ms_get_keys_released,
	get_char_pressed as ms_get_char_pressed,
	KeyCode as MsKeyCode
};

use super::key_code::{KeyCode, char_to_key};
use super::input::Input;

pub struct MacroquadInput {}

impl MacroquadInput {
	pub fn new() -> Self {
		Self{}
	}

	fn convert(&self, key_code: MsKeyCode) -> KeyCode {
		return match key_code {
			MsKeyCode::Backspace => KeyCode::Backspace,
			MsKeyCode::RightControl => KeyCode::Control,
			MsKeyCode::LeftControl => KeyCode::Control,
			MsKeyCode::Enter => KeyCode::Enter,
			MsKeyCode::Escape => KeyCode::Escape,
			MsKeyCode::Space => KeyCode::Space,
			MsKeyCode::Key0 => KeyCode::Key0,
			MsKeyCode::Key1 => KeyCode::Key1,
			MsKeyCode::Key2 => KeyCode::Key2,
			MsKeyCode::Key3 => KeyCode::Key3,
			MsKeyCode::Key4 => KeyCode::Key4,
			MsKeyCode::Key5 => KeyCode::Key5,
			MsKeyCode::Key6 => KeyCode::Key6,
			MsKeyCode::Key7 => KeyCode::Key7,
			MsKeyCode::Key8 => KeyCode::Key8,
			MsKeyCode::Key9 => KeyCode::Key9,
			MsKeyCode::A => KeyCode::A,
			MsKeyCode::B => KeyCode::B,
			MsKeyCode::C => KeyCode::C,
			MsKeyCode::D => KeyCode::D,
			MsKeyCode::E => KeyCode::E,
			MsKeyCode::F => KeyCode::F,
			MsKeyCode::G => KeyCode::G,
			MsKeyCode::H => KeyCode::H,
			MsKeyCode::I => KeyCode::I,
			MsKeyCode::J => KeyCode::J,
			MsKeyCode::K => KeyCode::K,
			MsKeyCode::L => KeyCode::L,
			MsKeyCode::M => KeyCode::M,
			MsKeyCode::N => KeyCode::N,
			MsKeyCode::O => KeyCode::O,
			MsKeyCode::P => KeyCode::P,
			MsKeyCode::Q => KeyCode::Q,
			MsKeyCode::R => KeyCode::R,
			MsKeyCode::S => KeyCode::S,
			MsKeyCode::T => KeyCode::T,
			MsKeyCode::U => KeyCode::U,
			MsKeyCode::V => KeyCode::V,
			MsKeyCode::W => KeyCode::W,
			MsKeyCode::X => KeyCode::X,
			MsKeyCode::Y => KeyCode::Y,
			MsKeyCode::Z => KeyCode::Z,
			_ => KeyCode::Space,
		}
	}
}

impl Input for MacroquadInput {
	fn get_keys_released(&self) -> HashSet<KeyCode> {
		return ms_get_keys_released().iter().map(|&key| self.convert(key)).collect();
	}

	fn get_key_pressed(&self) -> Option<KeyCode> {
		match ms_get_char_pressed() {
			Some(ch) => return Some(char_to_key(ch)),
			None => return None,
		}
	}

	fn update(&self) {}
}
