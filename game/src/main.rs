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
mod interface;
mod cli_interface;
use interface::Interface;
use cli_interface::CliInterface;
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


#[macroquad::main("MyGame")]
async fn main() {
// fn main() {
	// let xdg_dirs = xdg::BaseDirectories::with_prefix("simpleRPG");


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

	data::main::load_data(&mut game.components);

	game.setup_scene();
	interface.render_detailed(&game);
	interface.render_actions(&game);
	loop {
		// Reporting
		if Instant::now() - lastsec >= Duration::from_secs(1) {
            // println!("{:?}", game.state.state_changes);
			lastsec = Instant::now();
        }

		interface.render(&game);

        next_frame().await;
		match interface.check_input(&game) {
			Ok(response) => {
				match response {
					Some(action) => {
						game.state.last_action_type = action.action_type.clone();
						match action.action_type {
							ActionType::CHECK_INVENTORY => (),
							ActionType::GO => {
								game.state.current_location =
									game.components.destinations[action.arg_1.unwrap() - game.components.exits_start]
							},
							ActionType::LOOK => interface.render_detailed(&game),
							ActionType::TAKE => {
								let id = action.arg_1.unwrap();
								let index = game.components.locations[game.state.current_location].iter().position(|eid| *eid == id).unwrap();
								game.components.locations[game.state.current_location].remove(index);
								game.components.locations[game.components.inventory_id].push(id);
								// record change to world state
								game.state.update_location(game.components.uuids[id], game.components.inventory_id);
							},
							ActionType::TALK => ()
						}

						game.setup_scene();
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
