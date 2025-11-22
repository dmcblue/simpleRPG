// Writes out the conversations data

// int
use std::fs::File;
use std::io::Write;
use std::clone::Clone;
use regex::Regex;

// ext
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationNode {
	pub id: usize,
	pub prompt: String,
	pub response: String,
	pub after: Vec<String>,
	pub enabled: bool,
	pub prompts: Vec<ConversationNode>,
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
			use super::event::{Event, EventType};\n\
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
				\t\t\tafter: Vec::new(),\n\
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

	pub fn render_event_type(&mut self, event_type_str: &str) -> &str {
		match event_type_str {
			"Enable" => { return "EventType::ENABLE_CONVERSATION"; },
			_ => { panic!("Unknown EventType '{}'", event_type_str ) },
		}
	}

	pub fn render_after(&mut self, after: String) -> String {
		let mut ss = after.split(" ");
		let event_type_str = ss.next().unwrap();
		let arg_1_str = ss.next().unwrap();
		let event_type: &str = self.render_event_type(event_type_str);

		let regex = Regex::new(r"\d{10}").unwrap();
		if !regex.is_match(arg_1_str) {
			panic!("Bad UUID '{}'", arg_1_str)
		}

		return format!("Event{{ event_type: {}, arg_1: Some({}) }}", event_type, arg_1_str);
	}

	// needs to be iterative to not be awful
	pub fn render_conversation(
		&mut self,
		conversation: &ConversationNode,
		depth: String
	) {
		let mut actions: String = String::from("vec![");
		for event in conversation.after.clone() {
			actions = actions + self.render_after(event).as_str();
		}
		actions = actions + "]";
		let _ = self.file_handle.write_all(
			format!(
				"{}\t\t\t\tConversationNode{{\n\
				{}\t\t\t\t\tid: {},\n\
				{}\t\t\t\t\tenabled: {},\n\
				{}\t\t\t\t\tis_root: false,\n\
				{}\t\t\t\t\tafter: {},\n\
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
				actions,
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
				"{}\t\t\t\t\t],\n\
				{}\t\t\t\t}},\n",
				depth,
				depth
			).as_bytes()
		);
	}
}
