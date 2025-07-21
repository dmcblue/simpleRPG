use std::collections::HashMap;

pub struct Components<'a> {
    pub current_entities: u32,
    pub descriptions: HashMap<u32, &'a str>,
    pub destinations: HashMap<u32, u32>,
    pub locations: HashMap<u32, Vec<u32>>,
    pub names: HashMap<u32, &'a str>,
}