// int
use std::fs::File;
use std::io::Write;
use std::clone::Clone;

// ext
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ConversationNode {
	id: usize,
	prompt: String,
	response: String,
	after: Option<String>,
	enabled: bool,
	prompts: Vec<ConversationNode>,
}

pub struct ConversationsFile {
	file_handle: File
}

impl ConversationsFile {
	pub fn new() -> Self {
		Self {
			file_handle: File::create("../game/src/data/conversations_impl.rs").unwrap(),
		}
	}

	pub fn begin(&mut self) {
		let _ = self.file_handle.write_all(
			b"use super::components::Components;\n\
			use super::conversations::ConversationNode;\n\
			\n\
			pub fn load_conversations(components: &mut Components) {\n\
			\tcomponents.conversations = [\n"
		);
	}

	pub fn end(&mut self) {
		let _ = self.file_handle.write_all(
			b"\t];\n}\n"
		);
	}

	pub fn open_root(&mut self, uuid: usize) {
		let _ = self.file_handle.write_all(
			format!(
				"\t\tConversationNode{{\n\
				\t\t\tid: {},\n\
				\t\t\tenabled: true,\n\
				\t\t\tis_root: true,\n\
				\t\t\tprompt: String::new(),\n\
				\t\t\tresponse: String::new(),\n\
				\t\t\tprompts: vec![\n\
				",
				uuid
			).as_bytes()
		);
	}

	pub fn close_root(&mut self) {
		let _ = self.file_handle.write_all(
			b"\t\t\t]\n\t\t},\n"
		);
	}


	// needs to be iterative to not be awful
	pub fn render_conversation(
		&mut self,
		conversation: &ConversationNode,
		depth: String
	) {
		let _ = self.file_handle.write_all(
			format!(
				"{}\t\t\t\tConversationNode{{\n\
				{}\t\t\t\t\tid: {},\n\
				{}\t\t\t\t\tenabled: {},\n\
				{}\t\t\t\t\tis_root: false,\n\
				{}\t\t\t\t\tprompt: String::from(\"{}\"),\n\
				{}\t\t\t\t\tresponse: String::from(\"{}\"),\n\
				{}\t\t\t\t\tprompts: vec![\n",
				depth,
				depth,
				conversation.id,
				depth,
				conversation.enabled,
				depth,
				depth,
				conversation.prompt,
				depth,
				conversation.response,
				depth,
			).as_bytes()
		);
		for c in conversation.prompts.clone() {
			self.render_conversation(
				&c,
				format!("\t\t{}", depth)
			);
		}
		let _ = self.file_handle.write_all(
			format!(
				"{}\t\t\t\t\t]\n\
				{}\t\t\t\t}}\n",
				depth,
				depth
			).as_bytes()
		);
	}
}
