// std

// ext
use crossterm::terminal::size;
use ratatui::{
    DefaultTerminal,
    // Frame,
    layout::Rect,
    style::{
		Style,
		// Stylize
	},
    // text::Line,
    widgets::{Block, Clear, Paragraph},
};

// int
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
	fn init(&self, frame: &RenderFrame) {
		match size() {
			Ok(tuple) => {
				if tuple.0 < <usize as TryInto<u16>>::try_into(frame.width).unwrap() || tuple.1 < <usize as TryInto<u16>>::try_into(frame.height).unwrap() {
					panic!(
						"Terminal size ({}, {}) not sufficient. Must be at least ({}, {}).",
						tuple.0,
						tuple.1,
						frame.width,
						frame.height,
					)
				}
			},
			Err(_e) => {}
		}
	}

	fn render(&mut self, frame: &mut RenderFrame) {
		let area = Rect::new(
			1, // x
			1, // y
			<usize as TryInto<u16>>::try_into(frame.width).unwrap() + 1, // width
			<usize as TryInto<u16>>::try_into(frame.height).unwrap() + 1 // height
		); // x, y, width, height
		// log::info!("{} = {} = {} = {}", frame.width, frame.height, <usize as TryInto<u16>>::try_into(frame.width).unwrap() + 1, <usize as TryInto<u16>>::try_into(frame.height).unwrap() + 1);
		let block = Block::default()
			// .title("Rectangle")
			// .borders(ratatui::widgets::Borders::ALL)
			.style(Style::default().fg(ratatui::style::Color::Green));
		let text: String = frame.each_line().map(|cs| cs.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
		// log::info!("'{}'::{:?}", text, size());
		let paragraph = Paragraph::new(text).block(block);
		let _ = self.terminal.draw(|r_frame| {
        	r_frame.render_widget(Clear, area);
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
