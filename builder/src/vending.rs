// Builds in the Vending data files

// std
use std::fs::File;
use std::io::Write;
use std::clone::Clone;

// ext
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Vending {
	pub id: usize,
	pub items: Vec<VendItem>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VendItem {
	pub id: usize,
	pub price: String
}

pub struct VendingsFile {
	file_handle: File
}

impl VendingsFile {
	pub fn new() -> Self {
		Self {
			file_handle: File::create("../game/src/data/vending_impl.rs").unwrap(),
		}
	}

	pub fn begin(&mut self) {
		let _ = self.file_handle.write_all(
			b"use super::components::{Components};\n\
			use super::vending::{Price, Vending, VendItem};\n\
			\n\
			pub fn load_vendings(components: &mut Components) {\n\
			\tcomponents.vendings = [\n"
		);
	}

	pub fn end(&mut self) {
		let _ = self.file_handle.write_all(
			b"\t];\n}\n"
		);
	}

	pub fn render_vending(&mut self, vending: &Vending) {
		let _ = self.file_handle.write_all(
			format!(
				"\t\tVending{{\n\
				\t\t\tid: {},\n\
				\t\t\titems: vec![\n\
				",
				vending.id
			).as_bytes()
		);
		for vend_item in &vending.items {
			let _ = self.file_handle.write_all(
				format!(
					"\t\t\t\tVendItem{{\n\
					\t\t\t\t\tid: {},\n\
					\t\t\t\t\tprice: {},\n\
					\t\t\t\t}}\n",
					vend_item.id,
					self.render_vend_item_price(&vend_item.price)
				).as_bytes()
			);
		}
		let _ = self.file_handle.write_all(
			b"\t\t\t]\n\t\t},\n"
		);
	}

	pub fn render_vend_item_price(&self, price: &String) -> String {
		let parts: Vec<&str> = price.split(" ").collect();
		match *parts.get(0).unwrap() {
			"range" => {
				return format!(
					"Price::Range({}, {})",
					parts.get(1).unwrap(),
					parts.get(2).unwrap()
				);
			}
			_ => { return String::new(); }
		}

	}
}
