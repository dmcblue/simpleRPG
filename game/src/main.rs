use std::io::{self, Write};
use std::fs::{File};
use std::collections::HashMap;
// use simple::{Rect, Window};
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::time::{Instant, Duration};
use chrono::Utc;

mod action;
use action::{Action, ActionType};
mod macroquad_interface;
use macroquad_interface::MacroquadInterface;

mod data;
mod game;
use game::Game;
mod game_action;
use game_action::GameAction;
mod scene;
use scene::Scene;
mod state;
use state::State;
mod mode;
use mode::Mode;
mod main_menu_action;
use main_menu_action::MainMenuAction;


#[macroquad::main("MyGame")]
async fn main() {
	let mut interface = MacroquadInterface{
		text: VecDeque::new()
	};
	let mut game = Game {
		components: data::make_components(),
		scene: Scene{
			location_id: 0,
			entity_ids: Vec::new(),
			exit_ids: Vec::new(),
			takeable_item_ids: Vec::new(),
			actions: Vec::new(),
		},
		state: State {
			current_location: data::main::get_start_location_id(),
			last_action_type: ActionType::GO,
			state_changes: HashMap::new()
		}
	};
	let mut lastsec = Instant::now();
	let mut mode: Mode = Mode::MAIN_MENU;
	change_mode(&mut mode, &mut game, &mut interface);

	data::main::load_data(&mut game.components);

	// game.setup_scene();
	// interface.render_location_detailed(&game);
	// interface.render_actions(&game);
	loop {
		// Reporting
		if Instant::now() - lastsec >= Duration::from_secs(1) {
            // println!("{:?}", game.state.state_changes);
			lastsec = Instant::now();
        }

		// render
		match mode {
			Mode::MAIN_MENU => {
				interface.render_main_menu();

				next_frame().await;
				match interface.check_input_main_menu() {
					Some(MainMenuAction::NEW_GAME) => {
						mode = Mode::PLAY;
						change_mode(&mut mode, &mut game, &mut interface);
					},
					Some(MainMenuAction::LOAD_GAME) => {},
					Some(MainMenuAction::QUIT) => {
						println!("Goodbye!");
						break;
					},
					None => ()
				}
			},
			Mode::PLAY => {
				interface.render_play(&game);

				next_frame().await;
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
								save(&game);
								interface.render_save();
							}
						}
					}
				}
			}
		}
    }
}

fn change_mode(mode: &Mode, game: &mut Game, interface: &mut MacroquadInterface) {
	interface.change_mode(mode);
	match *mode {
		Mode::MAIN_MENU => {

		},
		Mode::PLAY => {
			// load game somewhere else?
			game.setup_scene();
			interface.render_location_detailed(&game);
			interface.render_actions(&game);
		},
	}
}

fn save(game: &Game) {
	let time = Utc::now();
	let xdg_dirs = xdg::BaseDirectories::with_prefix("simpleRPG");
	let save_path = xdg_dirs.place_data_file(format!("{}.sv", time.timestamp())).unwrap();

	match File::create(save_path) {
		Ok(save_file) => {
			write!(&save_file, "{}", game.state.state_changes_to_file_content());
		},
		Err(e) => { println!("{:?}", e); }
	}
}
