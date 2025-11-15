use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Items {
	// uuid -> quantity
	pub _items: HashMap<usize, usize>,
}

impl Items {
	pub fn new() -> Self {
		Self {
			_items: HashMap::new(),
		}
	}

	pub fn add(&mut self, item_uuid: usize, quantity: usize) -> usize {
		match self._items.get(&item_uuid) {
			Some(current_quantity) => {
				self._items.insert(item_uuid, current_quantity + quantity);
			},
			None => {
				self._items.insert(item_uuid, quantity);
			},
		}

		return *self._items.get(&item_uuid).unwrap();
	}

	pub fn any(&self, item_uuid: usize) -> bool {
		match self._items.get(&item_uuid) {
			Some(current_quantity) => {
				return *current_quantity > 0;
			},
			None => {
				return false;
			},
		}
	}

	pub fn drain(&mut self) {
		self._items.drain();
	}

	pub fn how_many(&self, item_uuid: usize) -> usize {
		match self._items.get(&item_uuid) {
			Some(current_quantity) => {
				return *current_quantity;
			},
			None => {
				return 0;
			},
		}
	}

	pub fn iter(&self) -> std::collections::hash_map::Iter<'_, usize, usize> {
		self._items.iter()
	}

	pub fn into_iter(&mut self) -> std::collections::hash_map::IntoIter<usize, usize> {
		self._items.clone().into_iter()
	}

	pub fn remove(&mut self, item_uuid: usize, quantity: usize) -> Result<usize, &str> {
		match self._items.get(&item_uuid) {
			Some(current_quantity) => {
				if *current_quantity < quantity {
					return Err("Negative amount");
				}
				self._items.insert(item_uuid, *current_quantity - quantity);
			},
			None => {
				return Err("Negative amount");
			},
		}

		return Ok(*self._items.get(&item_uuid).unwrap());
	}

	pub fn to_hash_map(&self) -> HashMap<usize, usize> {
		return self._items.clone();
	}
}
