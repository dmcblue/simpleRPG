// std
use std::collections::HashSet;

// int
use super::key_code::KeyCode;

pub trait Input {
	// key presses as keycodes,
	// inputs as chars or strings
	// let Some(ch) = get_char_pressed()
	// let key_set = get_keys_released();

	// 	if key_set.contains(&KeyCode::Q) {
	fn get_keys_released(&mut self) -> HashSet<KeyCode>;
	fn get_key_pressed(&mut self) -> Option<KeyCode>;
	fn update(&mut self);
}
