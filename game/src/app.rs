// std
use std::time::{Instant};

// ext

// int
use super::data::{
	load_conversations,
	load_data,
	load_vendings,
};
use super::game::Game;
use super::interface::Interface;
use super::mode::Mode;
use super::platform::Platform;
use crate::renderer::{
	Renderer,
};

pub struct App<'app> {
	pub game: &'app mut Game<'app>,
	pub interface: &'app mut Interface,
	pub is_running: bool,
	pub lastsec: Instant,
	pub mode: Mode,
	pub platform: Platform,
}

impl<'app> App<'app> {
	pub fn new(
		game: &'app mut Game<'app>,
		interface: &'app mut Interface,
	) -> App<'app> {
		App {
			game: game,
			interface: interface,
			is_running: true,
			lastsec: Instant::now(),
			mode: Mode::MAIN_MENU,
			platform: Platform::new(),
		}
	}

	pub fn initialize(&mut self) {
		self.platform.load();
		self.set_mode(Mode::MAIN_MENU);

		load_data(&mut self.game.components);
		load_conversations(&mut self.game.components);
		load_vendings(&mut self.game.components);

		self.interface.renderer.init();
	}

	pub fn set_mode(&mut self, mode: Mode) {
		self.mode = mode;
		self.interface.change_mode(&self.mode);
		match self.mode {
			Mode::LOAD => {
				self.platform.set_save_files();

				self.interface.render_save_files(self.platform.save_files.clone());
			},
			Mode::MAIN_MENU => {

			},
			Mode::PLAY => {
				// load game somewhere else?
				self.game.setup_scene();
				self.interface.render_location_detailed(self.game);
				self.interface.render_actions(self.game);
			},
			Mode::SAVE => {

			},
		}
	}

	pub fn terminate(&self) {
		self.interface.renderer.close();
	}
}
