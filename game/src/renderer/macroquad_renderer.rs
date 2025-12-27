use macroquad::prelude::{
	BLACK,
	// BLUE,
	GREEN,
	clear_background,
	// draw_rectangle,
	draw_text,
	next_frame,
	// screen_height,
	// screen_width,
};

use super::frame::Frame;
use super::renderer::Renderer;

pub struct MacroquadRenderer {}

impl MacroquadRenderer {
	pub fn new() -> Self {
		Self{}
	}
}

impl Renderer for MacroquadRenderer {
	fn init(&self, frame: &Frame) {}

	fn render(&mut self, frame: &mut Frame) {
		clear_background(BLACK);

		let mut i = 0.0;
		for line in frame.each_line() {
			let t: String = line.iter().collect();
			draw_text(t.as_str(), 10.0, 20.0 * (i + 1.0), 18.0, GREEN);

			i = i + 1.0;
		}
	}

	async fn update(&self) {
		next_frame().await;
	}

	fn close(&self) {}
}
