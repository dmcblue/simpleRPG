// std
use std::fs::File;
use std::io::Write;
use std::clone::Clone;

// ext

// int
use super::conversations::ConversationNode;

pub struct MainFile {
	file_handle: File
}

impl MainFile {
	pub fn new() -> Self {
		Self {
			file_handle: File::create("../game/src/data/main.rs").unwrap(),
		}
	}

	pub fn begin(&mut self) {
		let _ = self.file_handle.write_all(
			b"use super::components::Components;\n\
			// use super::vending::{Price, Vending, VendItem};\n\
			\n\
			pub fn load_data(components: &mut Components) {\n"
		);
	}

	pub fn end(&mut self, starting_location_uuid: usize) {
		let _ = self.file_handle.write_all(
			format!(
				"}}\n\npub fn get_start_location_uuid() -> usize {{ {} }}",
				starting_location_uuid,
			).as_bytes()
		);
	}

	pub fn render_at_location(&mut self, location_array_index: usize, entity_uuid: usize) {
		let _ = self.write_all(format!(
			"\tcomponents.locations[{}].push({});\n",
			location_array_index,
			entity_uuid
		));
	}

	pub fn render_conversation(&mut self, conversation: &ConversationNode) {
		let enabled_str = if conversation.enabled {
			"true"
		} else {
			"false"
		};

		let _ = self.write_all(format!(
			"\tcomponents.enabled.insert({}, {});\n",
			conversation.id,
			enabled_str,
		));

		for child in conversation.prompts.clone() {
			self.render_conversation(&child);
		}
	}

	pub fn render_name(&mut self, array_index: usize, name: String) {
		let _ = self.write_all(format!(
			"\tcomponents.names[{}] = \"{}\";\n",
			array_index,
			str::replace(name.as_str(), "\"", "\\\"")
		));
	}

	pub fn write_all(&mut self, s: String) {
		let _ = self.file_handle.write_all(s.as_bytes());
	}
}

