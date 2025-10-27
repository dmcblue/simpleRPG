use std::fs::{File};
use std::fs::OpenOptions;
use std::io::{Write};

pub struct Log {
	file: File,
}

impl Log {
	pub fn new(filename: &str) -> Option<Log> {
		match File::create(filename) {
			Ok(f) => {
				write!(&f, "");
			},
			Err(e) => { println!("{:?}", e); },
		}
		let file = OpenOptions::new()
			.write(true)
			.open(filename).ok();
		Some(Self{
			file: file.unwrap(),
		})
	}

	pub fn write(&mut self, st: &str) {
		let _ = self.file.write_all(st.as_bytes());
	}
}
