mod action;
mod app;
mod app_loop;
mod app_save_files;
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
mod log;
mod main_menu_action;
mod mode;
mod platform;
mod renderer;
mod scene;
mod state;
mod vending_action;

// std

// ext

// int
use app::App;
use game::Game;
use interface::Interface;
use log::Log;

// #[macroquad::main("MyGame")]
// async fn main() {
#[tokio::main]
async fn main() {
	let mut log: Log = Log::new("log.txt").expect("File issue");
	log.write("Starting up");
	let mut interface = Interface::new();
	let mut game: Game = Game::new();
	let mut app: App = App::new(&mut game, &mut interface, &mut log);

	app.initialize();
	app.run_loop().await;

	app.terminate();
}