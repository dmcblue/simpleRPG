use super::event::Event;

#[derive(Debug)]
pub struct ConversationNode {
	pub uuid: usize,
	pub is_root: bool,
	pub after: Vec<Event>,
	pub enabled: bool,
	pub prompt: String,
	pub response: String,
	pub prompts: Vec<ConversationNode>,
}

impl ConversationNode {
	pub fn new() -> Self {
		return Self {
			uuid: 0,
			enabled: true,
			is_root: true,
			after: Vec::new(),
			prompt: String::new(),
			response: String::new(),
			prompts: Vec::new(),
		};
	}
}
