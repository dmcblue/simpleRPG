#[derive(Debug)]
pub enum Price {
	Range(usize, usize)
}

#[derive(Debug)]
pub struct Vending {
	pub id: usize,
	pub items: Vec<VendItem>,
}

#[derive(Debug)]
pub struct VendItem {
	pub id: usize,
	pub price: Price,
}

impl Vending {
	pub fn new() -> Self {
		Self {
			id: 0,
			items: Vec::new(),
		}
	}
}
