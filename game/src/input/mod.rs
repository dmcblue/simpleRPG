pub mod input;
pub mod key_code;
// pub mod macroquad_input;
pub mod ratatui_input;

pub use input::Input;
// pub use macroquad_input::MacroquadInput;
pub use ratatui_input::RatatuiInput;
pub use key_code::{
	KeyCode,
	NUMBERS,
	TYPEABLE,
	// char_to_key,
	key_to_char
};
