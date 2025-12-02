#[derive(Clone, Copy, Debug)]
pub struct Price {
	pub item_uuid: usize,
	pub quantity: usize,
}

#[derive(Debug)]
pub struct Vending {
	pub uuid: usize,
	pub items: Vec<VendItem>,
}

#[derive(Debug)]
pub struct VendItem {
	pub uuid: usize,
	pub price: Price,
}

impl Vending {
	pub fn new() -> Self {
		Self {
			uuid: 0,
			items: Vec::new(),
		}
	}
}
