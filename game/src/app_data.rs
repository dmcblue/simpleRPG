use std::env;
use std::fs::{read_dir};

pub struct AppData {
	directory_separator: String,
	pub save_dir: String,
	pub save_files: Vec<String>
}
impl AppData {
	pub fn new () -> Self {
		Self {
			directory_separator: String::new(),
			save_dir: String::new(),
			save_files: Vec::new()
		}
	}

	pub fn load(&mut self) {
		self.directory_separator = String::from("/");
		match env::home_dir() {
			Some(home_path) => {
				// TODO run on different systems
				self.save_dir = format!("{}/.local/share/simpleRPG/", home_path.display());
			},
			None => panic!("Impossible to get your home dir!"),
		}
	}

	pub fn set_save_files(&mut self) {
		self.save_files = Vec::new();
		for path in read_dir(format!("{}", self.save_dir)).unwrap() {
			let f = path.unwrap().path().display().to_string();
			let (_, file_name) = f.rsplit_once(&self.directory_separator).unwrap();
			self.save_files.push(String::from(file_name));
		}
	}
}
