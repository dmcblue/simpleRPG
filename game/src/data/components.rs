use std::collections::HashMap;

pub struct Components<'a> {
    pub current_entities: usize,
    pub descriptions: HashMap<usize, &'a str>,
    pub destinations: HashMap<usize, usize>,
    pub locations: HashMap<usize, Vec<usize>>,
    pub names: HashMap<usize, &'a str>,
}