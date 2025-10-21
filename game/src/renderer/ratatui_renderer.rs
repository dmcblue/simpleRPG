use ratatui::{
    DefaultTerminal,
    // Frame,
    layout::Rect,
    style::{
		Style,
		// Stylize
	},
    // text::Line,
    widgets::{Block, Paragraph},
};

use super::frame::Frame as RenderFrame;
use super::renderer::Renderer;

pub struct RatatuiRenderer {
	terminal: DefaultTerminal,
}

impl RatatuiRenderer {
	pub fn new() -> Self {
		Self{
			terminal: ratatui::init(),
		}
	}
}

impl Renderer for RatatuiRenderer {
	fn init(&self) {}

	fn render(&mut self, frame: &mut RenderFrame) {
		let area = Rect::new(1, 1, <usize as TryInto<u16>>::try_into(frame.width).unwrap() + 1, <usize as TryInto<u16>>::try_into(frame.height).unwrap() + 1); // x, y, width, height
		let block = Block::default()
			// .title("Rectangle")
			// .borders(ratatui::widgets::Borders::ALL)
			.style(Style::default().fg(ratatui::style::Color::Green));
		let text: String = frame.each_line().map(|cs| cs.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
		let paragraph = Paragraph::new(text)
			.block(block);
		self.terminal.draw(|r_frame| {
        	r_frame.render_widget(paragraph, area);
		});
	}

	async fn update(&self) {
		// next_frame().await;
		// self.terminal.draw(|r_frame| {
		// 	// self.render(frame);
		// 	let area = Rect::new(1, 1, frame.width.try_into().unwrap(), frame.height.try_into().unwrap()); // x, y, width, height
		// 	let block = Block::default()
		// 		.title("Rectangle")
		// 		.borders(ratatui::widgets::Borders::ALL)
		// 		.style(Style::default().fg(ratatui::style::Color::Yellow));
		// 	let text: String = frame.each_line().map(|cs| cs.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
		// 	let paragraph = Paragraph::new(text)
        //     	.block(block);
        // 	r_frame.render_widget(paragraph, area);
		// });
	}

	fn close(&self) {
		ratatui::restore();
	}
}
