use super::action::Action;
use super::game::Game;

pub trait Interface {
	fn init(&self);

	fn get_input(&self) -> String;

	fn render(&mut self, game: &Game);

	fn render_detailed(&mut self, game: &Game);

	fn render_action(&self, game: &Game, action: &Action) -> String;

	fn open_inventory(&mut self, game: &Game);
}