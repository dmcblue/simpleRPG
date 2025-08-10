use std::io::{Write};
use std::env;
use std::fs::{File, read_dir, read_to_string};
use std::collections::HashMap;
use macroquad::prelude::*;
use std::time::{Instant, Duration};
use chrono::Utc;

mod action;
use action::{ActionType};
mod macroquad_interface;
use macroquad_interface::{
	MacroquadInterface,
};

mod app_data;
use app_data::AppData;
mod data;
use data::Components;
mod game;
use game::Game;
mod game_action;
use game_action::GameAction;
mod scene;
use scene::Scene;
mod state;
use state::{Field, State};
mod mode;
use mode::Mode;
mod main_menu_action;
use main_menu_action::MainMenuAction;
mod constants;

#[macroquad::main("MyGame")]
async fn main() {
	let mut app_data: AppData = AppData::new();
	app_data.load();

	let mut interface = MacroquadInterface::new();
	let mut game: Game = Game::new();
	let mut lastsec = Instant::now();
	let mut mode: Mode = Mode::MAIN_MENU;
	change_mode(&mut mode, &mut app_data, &mut game, &mut interface);

	data::main::load_data(&mut game.components);

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
				interface.render_play();
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
								read_file(&app_data, file_name.as_str(), &mut game.state, &game.components);
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
				match interface.check_input_play(&game) {
					Ok(response) => {
						match response {
							Some(action) => {
								game.handle_action(action);
								interface.render_action_taken(&game, &action);
								interface.render_actions(&game);
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

fn change_mode(mode: &Mode, app_data: &mut AppData, game: &mut Game, interface: &mut MacroquadInterface) {
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

fn read_file(app_data: &AppData, file_name: &str, state: &mut State, components: &Components) {
	let save_path = format!("{}{}", app_data.save_dir, file_name);
	let contents = read_to_string(save_path).unwrap();
	state.load_from_file(contents, components);
}

fn replay_state_changes(state: &State, components: &mut Components) {
	for (entity_uuid, changes) in state.state_changes.iter() {
		for (field, value) in changes {
			match field {
				Field::LOCATION => {
					components.move_to(*entity_uuid, *value);
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
			let _ = write!(&save_file, "{}", game.state.state_changes_to_file_content(name));
		},
		Err(e) => { println!("{:?}", e); }
	}
}
