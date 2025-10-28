mod action;
mod app;
mod app_loop;
mod app_save_files;
mod app_handle_input;
mod constants;
mod conversation_action;
mod data;
mod game;
mod game_action;
mod game_mode;
mod input;
mod interface;
mod interface_input;
mod interface_render;
mod main_menu_action;
mod mode;
mod platform;
mod renderer;
mod scene;
mod state;
mod vending_action;

// std

// ext
use log4rs;

// int
use app::App;
use game::Game;
use interface::Interface;
use log::Log;

// #[macroquad::main("MyGame")]
// async fn main() {
#[tokio::main]
async fn main() {
	log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
	log::info!("Starting up");
	let mut interface = Interface::new();
	let mut game: Game = Game::new();
	let mut app: App = App::new(&mut game, &mut interface);

	app.initialize();
	app.run_loop().await;

	app.terminate();
}
