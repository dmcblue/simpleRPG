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
						// go back
						if i < 0 {
							self.set_mode(Mode::MAIN_MENU);
						} else {
							let u = (i as usize) - 1;
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
					},
					None => ()
				}
			},
			Mode::MAIN_MENU => {
				match self.interface.check_input_main_menu() {
					Some(MainMenuAction::NEW_GAME) => {
						self.set_mode(Mode::PLAY);
					},
					Some(MainMenuAction::LOAD_GAME) => {
						self.set_mode(Mode::LOAD);
					},
					Some(MainMenuAction::QUIT) => {
						println!("Goodbye!");
						self.is_running = false;
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
										self.game.handle_action(action, &mut self.log);
										self.interface.render_action_taken(&self.game, &action);
										match self.game.mode {
											GameMode::EXPLORE => {
												self.interface.render_actions(&self.game);
											},
											GameMode::TALK => {
												self.interface.render_conversation(
													self.game.get_conversation()
												);
											},
											GameMode::VEND => {
												self.interface.render_vending(
													&self.game.components.vendings[self.game.state.current_vending_id],
													&self.game.components
												);
											}
										}
									},
									None => ()
								}
							},
							Err(st) => {
								match st {
									GameAction::QUIT => {
										println!("Goodbye!");
										self.is_running = false;
									},
									GameAction::SAVE => {
										self.set_mode(Mode::SAVE);
									},
									_ => ()
								}
							}
						}
					},
					GameMode::TALK => {
						match self.interface.check_input_talk(&self.game) {
							Ok(conversation_action) => {
								match conversation_action {
									ConversationAction::ADD(i) => {
										self.game.state.current_conversation.path.push(i);
										self.interface.render_conversation_response(&self.game.get_conversation().response);
										if self.game.get_conversation().prompts.len() < 2 {
											self.game.state.current_conversation.path.pop();
										}
										self.interface.render_conversation(
											self.game.get_conversation()
										);
									},
									ConversationAction::BACK => {
										if self.game.state.current_conversation.path.len() > 0 {
											self.game.state.current_conversation.path.pop();
											self.interface.render_conversation(
												self.game.get_conversation()
											);
										} else {
											self.game.mode = GameMode::EXPLORE;
											self.game.setup_scene();
											self.interface.render_location_detailed(&self.game);
											self.interface.render_actions(&self.game);
										}
									},
									ConversationAction::END => {
										self.game.mode = GameMode::EXPLORE;
										self.game.setup_scene();
										self.interface.render_location_detailed(&self.game);
										self.interface.render_actions(&self.game);
									},
									ConversationAction::NONE => {}
								}
							},
							Err(game_action) => {
								match game_action {
									GameAction::QUIT => {
										println!("Goodbye!");
										self.is_running = false;
									},
									GameAction::SAVE => {
										self.set_mode(Mode::SAVE);
									},
									_ => ()
								}
							}
						}
					},
					GameMode::VEND => {
						match self.interface.check_input_vend(&self.game, &self.game.components.vendings[self.game.state.current_vending_id]) {
							Ok(vending_action) => {
								match vending_action {
									VendingAction::BUY(i) => {
										self.log.write(&format!("Buy!: {}", i).to_string());
										// println!("Buy!: {}", i);
										// vending.items
										let _item = self.game.components.vendings[self.game.state.current_vending_id].items.get(i).unwrap();

										// Action{
										// 	action_type: ActionType::TAKE,
										// 	arg_1: Some(*item_id),
										// 	..Default::default()
										// }
									},
									VendingAction::ERROR(message) => {
										self.log.write(&format!("Error!: {}", message).to_string());
									},
									VendingAction::NONE => {}
								}
							},
							Err(game_action) => {
								match game_action {
									GameAction::QUIT => {
										println!("Goodbye!");
										self.is_running = false;
									},
									GameAction::SAVE => {
										self.set_mode(Mode::SAVE);
									},
									_ => ()
								}
							}
						}
					}
				}
			},
			Mode::SAVE => {
				match self.interface.check_input_save() {
					Some(s) => {
						self.log.write(&format!("{}", s).to_string());
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