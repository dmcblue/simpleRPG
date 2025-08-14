mod action;
mod app_data;
mod constants;
mod conversation_action;
mod data;
mod game;
mod game_action;
mod game_mode;
mod interface;
mod interface_input;
mod interface_render;
mod main_menu_action;
mod mode;
mod scene;
mod state;

// std
use std::fs::{File, read_to_string};
use std::io::{Write};
use std::time::{Instant, Duration};

// ext
use chrono::Utc;
use macroquad::prelude::*;

// int
use app_data::AppData;
use conversation_action::ConversationAction;
use data::{Components, load_conversations, load_data, Price, Vending, VendItem};
use game::Game;
use game_action::GameAction;
use game_mode::GameMode;
use interface::Interface;
use main_menu_action::MainMenuAction;
use mode::Mode;
use state::{Field, State};

#[macroquad::main("MyGame")]
async fn main() {
	let mut app_data: AppData = AppData::new();
	app_data.load();

	let mut interface = Interface::new();
	let mut game: Game = Game::new();
	let mut lastsec = Instant::now();
	let mut mode: Mode = Mode::MAIN_MENU;
	change_mode(&mut mode, &mut app_data, &mut game, &mut interface);

	load_data(&mut game.components);
	load_conversations(&mut game.components);

	loop {
		// Reporting
		if Instant::now() - lastsec >= Duration::from_secs(1) {
            // println!("{:?}", game.state.state_changes);
			lastsec = Instant::now();
        }

		// render
		match mode {
			Mode::LOAD => {
				interface.render_load();
			},
			Mode::MAIN_MENU => {
				interface.render_main_menu();
			},
			Mode::PLAY => {
				match game.mode {
					GameMode::EXPLORE => {
						interface.render_play();
					},
					GameMode::TALK => {
						interface.render_play();
					}
				}
			},
			Mode::SAVE => {
				interface.render_save();
			}
		}

		next_frame().await;

		// update
		match mode {
			Mode::LOAD => {
				match interface.check_input_load() {
					Some(i) => {
						// go back
						if i < 0 {
							mode = Mode::MAIN_MENU;
							change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
						} else {
							let u = (i as usize) - 1;
							if u < app_data.save_files.len() {
								// load
								let file_name = app_data.save_files[u].clone();
								read_file(&app_data, file_name.as_str(), &mut game.state, &mut game.components);
								replay_state_changes(&game.state, &mut game.components);
								mode = Mode::PLAY;
								change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
							} else {
								interface.error(&mode, "Bad file index")
							}
						}
					},
					None => ()
				}
			},
			Mode::MAIN_MENU => {
				match interface.check_input_main_menu() {
					Some(MainMenuAction::NEW_GAME) => {
						mode = Mode::PLAY;
						change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
					},
					Some(MainMenuAction::LOAD_GAME) => {
						mode = Mode::LOAD;
						change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
					},
					Some(MainMenuAction::QUIT) => {
						println!("Goodbye!");
						break;
					},
					None => ()
				}
			},
			Mode::PLAY => {
				match game.mode {
					GameMode::EXPLORE => {
						match interface.check_input_play(&game) {
							Ok(response) => {
								match response {
									Some(action) => {
										game.handle_action(action);
										interface.render_action_taken(&game, &action);
										match game.mode {
											GameMode::EXPLORE => {
												interface.render_actions(&game);
											},
											GameMode::TALK => {
												interface.render_conversation(
													game.get_conversation()
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
										break;
									},
									GameAction::SAVE => {
										mode = Mode::SAVE;
										change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
									},
									_ => ()
								}
							}
						}
					},
					GameMode::TALK => {
						match interface.check_input_talk(&game) {
							Ok(conversation_action) => {
								match conversation_action {
									ConversationAction::ADD(i) => {
										game.state.current_conversation.path.push(i);
										interface.render_conversation_response(&game.get_conversation().response);
										if game.get_conversation().prompts.len() < 2 {
											game.state.current_conversation.path.pop();
										}
										interface.render_conversation(
											game.get_conversation()
										);
									},
									ConversationAction::BACK => {
										if game.state.current_conversation.path.len() > 0 {
											game.state.current_conversation.path.pop();
											interface.render_conversation(
												game.get_conversation()
											);
										} else {
											game.mode = GameMode::EXPLORE;
											game.setup_scene();
											interface.render_location_detailed(&game);
											interface.render_actions(&game);
										}
									},
									ConversationAction::END => {
										game.mode = GameMode::EXPLORE;
										game.setup_scene();
										interface.render_location_detailed(&game);
										interface.render_actions(&game);
									},
									ConversationAction::NONE => {}
								}
							},
							Err(game_action) => {
								match game_action {
									GameAction::QUIT => {
										println!("Goodbye!");
										break;
									},
									GameAction::SAVE => {
										mode = Mode::SAVE;
										change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
									},
									_ => ()
								}
							}
						}
					}
				}
			},
			Mode::SAVE => {
				match interface.check_input_save() {
					Some(s) => {
						println!("{}", s);
						save(&app_data, &game, s);
						mode = Mode::PLAY;
						change_mode(&mut mode, &mut app_data, &mut game, &mut interface);
						interface.render_saved();
					},
					None => (),
				}
			},
		}
    }
}

fn change_mode(mode: &Mode, app_data: &mut AppData, game: &mut Game, interface: &mut Interface) {
	interface.change_mode(mode);
	match *mode {
		Mode::LOAD => {
			app_data.set_save_files();

			interface.render_save_files(app_data.save_files.clone());
		},
		Mode::MAIN_MENU => {

		},
		Mode::PLAY => {
			// load game somewhere else?
			game.setup_scene();
			interface.render_location_detailed(&game);
			interface.render_actions(&game);
		},
		Mode::SAVE => {

		},
	}
}

fn read_file(app_data: &AppData, file_name: &str, state: &mut State, components: &mut Components) {
	let save_path = format!("{}{}", app_data.save_dir, file_name);
	let contents = read_to_string(save_path.clone()).unwrap();
	println!("{}:{}", save_path.clone(), contents.clone());
	state.load_from_file(contents, components);
}

fn replay_state_changes(state: &State, components: &mut Components) {
	for (entity_uuid, changes) in state.state_changes.iter() {
		for (field, value) in changes {
			match field {
				Field::LOCATION => {
					// assume item for now
					components.move_item_to(
						components.uuids[*entity_uuid],
						components.get_array_id(value)
					);
				}
			}
		}
	}
}

fn save(app_data: &AppData, game: &Game, name: String) {
	let time = Utc::now();

	let save_path = format!("{}{}.sv", app_data.save_dir, time.timestamp());

	match File::create(save_path) {
		Ok(save_file) => {
			let _ = write!(&save_file, "{}", game.state.state_changes_to_file_content(name, &game.components));
		},
		Err(e) => { println!("{:?}", e); }
	}
}
