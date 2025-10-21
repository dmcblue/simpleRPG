// use std::marker::Copy;

// use super::components::Components;

#[derive(Debug)]
pub struct ConversationNode {
	pub id: usize,
	pub is_root: bool,
	pub enabled: bool,
	pub prompt: String,
	pub response: String,
	pub prompts: Vec<ConversationNode>,}

impl ConversationNode {
	pub fn new() -> Self {
		return Self {
			id: 0,
			enabled: true,
			is_root: true,
			prompt: String::new(),
			response: String::new(),
			prompts: Vec::new(),
		};
	}
}
