// std
use std::fs::{File, read_to_string};
use std::io::{Write};

// ext
use chrono::Utc;

// int
use super::app::App;
use super::state::{Field};

impl<'app> App<'app> {
	pub fn read_file(&mut self, file_name: &str) {
		let save_path = format!(
			"{}{}",
			self.platform.save_dir,
			file_name
		);
		// handle exception
		let contents = read_to_string(save_path.clone()).unwrap();
		self.game.state.load_from_file(contents, &mut self.game.components);
	}

	pub fn replay_state_changes(&mut self) {
		for (entity_uuid, changes) in self.game.state.state_changes.iter() {
			for (field, value) in changes {
				match field {
					Field::LOCATION => {
						// assume item for now
						self.game.components.move_item_to(
							self.game.components.uuids[*entity_uuid],
							self.game.components.get_array_id(value),
						);
					}
				}
			}
		}
	}

	pub fn save(&mut self, name: String) {
		let time = Utc::now();

		let save_path = format!("{}{}.sv", self.platform.save_dir, time.timestamp());

		match File::create(save_path) {
			Ok(save_file) => {
				let _ = write!(&save_file, "{}", self.game.state.state_changes_to_file_content(name, &self.game.components));
			},
			Err(e) => { log::info!("{:?}", e); }
		}
	}
}
