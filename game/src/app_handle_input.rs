// std
use std::time::{Instant, Duration};

// ext

// int
use super::app::App;
use super::conversation_action::ConversationAction;
use super::game_action::GameAction;
use super::game_mode::GameMode;
use super::input::{Input};
use super::main_menu_action::MainMenuAction;
use super::mode::Mode;
use super::renderer::{
	Renderer,
};
use super::vending_action::VendingAction;

impl<'app> App<'app> {
	pub fn handle_input_load(&mut self, index: isize) {
		// go back
		if index < 0 {
			self.set_mode(Mode::MAIN_MENU);
		} else {
			let u = (index as usize) - 1;
			if u < self.platform.save_files.len() {
				// load
				let file_name = self.platform.save_files[u].clone();
				self.read_file(file_name.as_str());
				self.replay_state_changes();
				self.set_mode(Mode::PLAY);
			} else {
				self.interface.error(&self.mode, "Bad file index")
			}
		}
	}

	pub fn handle_input_main_menu(&mut self
}