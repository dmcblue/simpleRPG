// std

// ext

// int
use super::action::{Action, ActionType};
use super::app::App;
use super::conversation_action::ConversationAction;
use super::game_action::GameAction;
use super::game_mode::GameMode;
use super::main_menu_action::MainMenuAction;
use super::mode::Mode;
use super::vending_action::VendingAction;

impl<'app> App<'app> {
	pub fn handle_conversation_action(&mut self, conversation_action: ConversationAction) {
		match conversation_action {
			ConversationAction::ASK(i) => {
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
	}

	pub fn handle_game_action(&mut self, game_action: GameAction) {
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

	pub fn handle_main_menu_action(&mut self, action: MainMenuAction) {
		match action {
			MainMenuAction::NEW_GAME => {
				self.set_mode(Mode::PLAY);
			},
			MainMenuAction::LOAD_GAME => {
				self.set_mode(Mode::LOAD);
			},
			MainMenuAction::QUIT => {
				println!("Goodbye!");
				self.is_running = false;
			},
		}
	}

	pub fn handle_play_action(&mut self, action: Action) {
		self.interface.render_hr();
		self.game.handle_action(action);
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
					&self.game.components.vendings[self.game.state.current_vending_index],
					&self.game.components
				);
			}
		}
	}

	pub fn handle_vending_action(&mut self, vending_action: VendingAction) {
		self.interface.render_hr();
		match vending_action {
			VendingAction::BACK => {
				self.game.mode = GameMode::EXPLORE;
				self.game.setup_scene();
				self.interface.render_location_detailed(&self.game);
				self.interface.render_actions(&self.game);
			},
			VendingAction::BUY(i) => {
				log::info!("Buy!: {}", i);
				log::info!("From!: {}", self.game.state.current_vending_index);
				// vending.items
				let item_id =
					self.game.components.
						vendings[self.game.state.current_vending_index].
						items.get(i).unwrap().id;

				self.game.components.
						vendings[self.game.state.current_vending_index].
						items.remove(i);
				self.handle_play_action(Action{
					action_type: ActionType::TAKE,
					arg_1: Some(self.game.components.get_array_id(&item_id)),
					..Default::default()
				});

				if self.game.components.vendings[self.game.state.current_vending_index].items.len() > 0 {
					self.interface.render_vending(
						&self.game.components.vendings[self.game.state.current_vending_index],
						&self.game.components
					);
				} else {
					self.set_mode(Mode::PLAY);
				}
			},
			VendingAction::ERROR(message) => {
				log::info!("Error!: {}", message);
			},
			VendingAction::NONE => {}
		}
	}
}
