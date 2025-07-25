// use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_yml;


// #[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    #[serde(rename = "type")]
    entity_type: String,
    metaname: String,
    metadata: Vec<String>,
    starting_location: Option<usize>,
    id: Option<usize>,
    //#[serde(default)]
    name: Option<String>,
    description: Option<String>,
    // Exit Specific
    location: Option<usize>,
    to: Option<usize>,
}

fn main() {
    let mut paths = fs::read_dir("../data").unwrap();

    let mut file = File::create("../game/src/data/main.rs").unwrap();
    let _ = file.write_all(
        b"use super::components::Components;\n\n\
        pub fn load_data(components: &mut Components) {\n"
    );
    let mut index: usize = 0;
    let mut id_map: HashMap<usize, usize> = HashMap::new();
    let mut start_id: usize = 0;
    // TODO: Pre-hash everything and rearrange so locations are first
    // then you can use the component.locations as an array of Vec
    // instead of a hashmap
    for path in paths {
        let file_path = path.unwrap().path();
        let contents = fs::read_to_string(file_path).unwrap();
        let entity: Entity = serde_yml::from_str(&contents).unwrap();

        if entity.entity_type == "Game" {
            start_id = entity.starting_location.unwrap();
        } else {
            id_map.insert(entity.id.unwrap(), index);
            if entity.entity_type == "Location" {
                let _ = file.write_all(
                    format!(
                        "\tcomponents.locations.insert({}, Vec::new());\n", 
                        index
                    ).as_bytes()
                );
            }

            index = index + 1;
        }
    }
    paths = fs::read_dir("../data").unwrap();
    for path in paths {
        let file_path = path.unwrap().path();
        let contents = fs::read_to_string(file_path).unwrap();
        let entity: Entity = serde_yml::from_str(&contents).unwrap();

        // if entity.entity_type == "Game" {
        //     let _ = file.write_all(
        //         format!(
        //             "\tstate.current_location = {};\n", 
        //             id_map.get(&entity.starting_location.unwrap()).unwrap()
        //         ).as_bytes()
        //     );
        // } else 
        if entity.entity_type != "Game"{
            let id: &usize = id_map.get(&entity.id.unwrap()).unwrap();
            let _ = file.write_all(
                format!(
                    "\tcomponents.names.insert({}, \"{}\");\n", 
                    id, 
                    entity.name.unwrap()
                ).as_bytes()
            );
            let _ = file.write_all(
                format!(
                    "\tcomponents.descriptions.insert({}, \"{}\");\n", 
                    id, 
                    entity.description.unwrap()
                ).as_bytes()
            );
            // if entity.entity_type == "Location" {
            //     // noop
            // } else 
            if entity.entity_type == "Person" {
                let _ = file.write_all(
                    format!(
                        "\tcomponents.locations.get_mut(&{}).unwrap().push({});\n", 
                        id_map.get(&entity.starting_location.unwrap()).unwrap(),
                        id,
                    ).as_bytes()
                );
            } else if entity.entity_type == "Exit" {
                let _ = file.write_all(
                    format!(
                        "\tcomponents.locations.get_mut(&{}).unwrap().push({});\n",
                        id_map.get(&entity.location.unwrap()).unwrap(),
                        id,
                    ).as_bytes()
                );
                let _ = file.write_all(
                    format!(
                        "\tcomponents.destinations.insert({}, {});\n",
                        id, 
                        id_map.get(&entity.to.unwrap()).unwrap(),
                    ).as_bytes()
                );
            }
        }
    }
    let _ = file.write_all(format!("}}\n\npub fn get_start_location_id() -> usize {{ {} }}", id_map.get(&start_id).unwrap()).as_bytes());
}
