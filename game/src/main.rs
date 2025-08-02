use std::io::{self, Write};
use simple::{Rect, Window};
use macroquad::prelude::*;

mod action;
use action::{Action, ActionType};
mod interface;
mod cli_interface;
use interface::Interface;
use cli_interface::CliInterface;

mod data;
mod game;
use game::Game;
mod scene;
use scene::Scene;
mod state;
use state::State;


// #[macroquad::main("MyGame")]
// async fn main() {
fn main() {
	let interface = CliInterface{};
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
		}
	};

	data::main::load_data(&mut game.components);

	loop {
		game.setup_scene();
		interface.render(&game);
		let input: String = interface.get_input();
		if input == "quit" {
			// return Err("Goodbye!");
			println!("Goodbye!");
			break;
		} else {
			let mut i: usize = input.parse::<usize>().unwrap();
			i = i - 1;
			let action = &game.scene.actions[i];
			game.state.last_action_type = action.action_type.clone();
			match action.action_type {
				ActionType::CHECK_INVENTORY => interface.render_inventory(&game),
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
					// record change
				},
				ActionType::TALK => ()
				// _ => ()
			}
		}
	}

	// let mut app = Window::new("hello world", 1920, 1080);

	// app.set_color(255, 0, 255, 255);
	// app.draw_rect(Rect::new(
	// 	100,
	// 	110,
	// 	120,
	// 	130,
	// ));

	// while app.next_frame() {}

	// loop {
    //     clear_background(RED);

    //     draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    //     draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    //     draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

    //     next_frame().await
    // }
}
