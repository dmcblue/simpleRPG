use super::action::Action;
use super::game::Game;

pub trait Interface {
	fn init(&self);

	fn get_input(&self) -> String;

	fn render(&self, game: &Game);

	fn render_detailed(&self, game: &Game);

	fn render_action(&self, game: &Game, action: &Action) -> String;

	fn render_inventory(&self, game: &Game);
}