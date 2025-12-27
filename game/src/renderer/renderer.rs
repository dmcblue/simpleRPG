use super::frame::Frame;

pub trait Renderer {
	fn init(&self, frame: &Frame);

	fn render(&mut self, frame: &mut Frame);

	async fn update(&self);

	fn close(&self);
}
