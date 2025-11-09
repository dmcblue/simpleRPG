// std
use std::time::{Instant, Duration};

// ext

// int
use super::app::App;
use super::game_mode::GameMode;
use super::input::{Input};
use super::mode::Mode;
use super::renderer::{
	Renderer,
};

impl<'app> App<'app> {
	pub fn per_loop_reporting(&mut self) {
		// println!("{:?}", game.state.state_changes);
		self.lastsec = Instant::now();
	}

	pub async fn render(&mut self) {
		match self.mode {
			Mode::LOAD => {
				self.interface.render_load();
			},
			Mode::MAIN_MENU => {
				self.interface.render_main_menu();
			},
			Mode::PLAY => {
				match self.game.mode {
					GameMode::EXPLORE => {
						self.interface.render_play();
					},
					GameMode::TALK => {
						self.interface.render_play();
					},
					GameMode::VEND => {
						self.interface.render_play();
					}
				}
			},
			Mode::SAVE => {
				self.interface.render_save();
			}
		}
		self.interface.renderer.update().await;
	}

	pub async fn run_loop(&mut self) {
		while self.is_running {
			// Reporting
			if Instant::now() - self.lastsec >= Duration::from_secs(1) {
				self.per_loop_reporting();
			}

			self.render().await;

			// get input
			self.interface.input.update();

			self.update();
		}
	}

	pub fn update(&mut self) {
		match self.mode {
			Mode::LOAD => {
				match self.interface.check_input_load() {
					Some(i) => {
						self.handle_input_load(i);
					},
					None => ()
				}
			},
			Mode::MAIN_MENU => {
				match self.interface.check_input_main_menu() {
					Some(action) => {
						self.handle_main_menu_action(action)
					},
					None => ()
				}
			},
			Mode::PLAY => {
				match self.game.mode {
					GameMode::EXPLORE => {
						match self.interface.check_input_play(&self.game) {
							Ok(response) => {
								match response {
									Some(action) => {
										self.handle_play_action(action);
									},
									None => ()
								}
							},
							Err(game_action) => {
								self.handle_game_action(game_action);
							}
						}
					},
					GameMode::TALK => {
						match self.interface.check_input_talk(&self.game) {
							Ok(conversation_action) => {
								self.handle_conversation_action(conversation_action);
							},
							Err(game_action) => {
								self.handle_game_action(game_action);
							}
						}
					},
					GameMode::VEND => {
						match self.interface.check_input_vend(&self.game, &self.game.components.vendings[self.game.state.current_vending_index]) {
							Ok(vending_action) => {
								self.handle_vending_action(vending_action);
							},
							Err(game_action) => {
								self.handle_game_action(game_action);
							}
						}
					}
				}
			},
			Mode::SAVE => {
				match self.interface.check_input_save() {
					Some(s) => {
						self.save(s);
						self.set_mode(Mode::PLAY);
						self.interface.render_saved();
					},
					None => (),
				}
			},
		}
	}
}
