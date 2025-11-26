#![allow(unused_variables)]
#![allow(dead_code)]

// Universal constants
mod univ {
    pub struct Univ {
        pub gal_size: i32,
        pub starting_entities: i32,
    }
    pub static UNIV: Univ = Univ {
        gal_size: 10000,
        starting_entities: 10000,
    };
}

mod pos {
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }
    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Position { x, y }
        }
        pub fn random(max: i32) -> Self {
            let mut rng = rand::rng();
            Position {
                x: rng.random_range(0..max),
                y: rng.random_range(0..max),
            }
        }
        pub fn distance(&self, other: &Position) -> i32 {
            let dx = (other.x - self.x) as f64;
            let dy = (other.y - self.y) as f64;
            (dx * dx + dy * dy).sqrt() as i32
        }
        pub fn print(&self) {
            println!("Position: ({}, {})", self.x, self.y);
        }
    }
}

mod item_name {
    use serde::{Deserialize, Serialize};

    #[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub enum ItemName {
        MetalLow,
        MetalMid,
        MetalHigh,
        CompositeLow,
        CompositeMid,
        CompositeHigh,
        PolymerLow,
        PolymerMid,
        PolymerHigh,
    }

    pub static ITEM_NAMES: [ItemName; 9] = [
        ItemName::MetalLow,
        ItemName::MetalMid,
        ItemName::MetalHigh,
        ItemName::CompositeLow,
        ItemName::CompositeMid,
        ItemName::CompositeHigh,
        ItemName::PolymerLow,
        ItemName::PolymerMid,
        ItemName::PolymerHigh,
    ];
}

mod item_meta {
    use crate::item_name::ItemName;

    #[derive(Debug)]
    pub struct ItemMeta {
        pub fname: String,
        pub vol_pc: i32,
    }

    use std::collections::HashMap;
    pub struct InvListMeta(HashMap<ItemName, ItemMeta>);
    impl InvListMeta {
        pub fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(
                ItemName::MetalLow,
                ItemMeta {
                    fname: "Metal (low grade)".to_string(),
                    vol_pc: 10,
                },
            );
            map.insert(
                ItemName::MetalMid,
                ItemMeta {
                    fname: "Metals (mid grade)".to_string(),
                    vol_pc: 20,
                },
            );
            map.insert(
                ItemName::MetalHigh,
                ItemMeta {
                    fname: "Metals (high grade)".to_string(),
                    vol_pc: 30,
                },
            );
            map.insert(
                ItemName::CompositeLow,
                ItemMeta {
                    fname: "Composites (low grade)".to_string(),
                    vol_pc: 15,
                },
            );
            map.insert(
                ItemName::CompositeMid,
                ItemMeta {
                    fname: "Composites (mid grade)".to_string(),
                    vol_pc: 25,
                },
            );
            map.insert(
                ItemName::CompositeHigh,
                ItemMeta {
                    fname: "Composites (high grade)".to_string(),
                    vol_pc: 35,
                },
            );
            map.insert(
                ItemName::PolymerLow,
                ItemMeta {
                    fname: "Polymers (low grade)".to_string(),
                    vol_pc: 12,
                },
            );
            map.insert(
                ItemName::PolymerMid,
                ItemMeta {
                    fname: "Polymers (mid grade)".to_string(),
                    vol_pc: 22,
                },
            );
            map.insert(
                ItemName::PolymerHigh,
                ItemMeta {
                    fname: "Polymers (high grade)".to_string(),
                    vol_pc: 32,
                },
            );
            InvListMeta(map)
        }
        pub fn get(&self, item: &ItemName) -> Option<&ItemMeta> {
            self.0.get(item)
        }
        pub fn print(&self, item: &ItemName) {
            if let Some(meta) = self.get(item) {
                println!("{meta:?}");
            } else {
                println!("Item {:?} not found in metadata.", item);
            }
        }
    }

    use std::sync::LazyLock;
    pub static ILM: LazyLock<InvListMeta> = LazyLock::new(|| InvListMeta::new());
}

mod inv_store {
    use crate::item_meta::ILM;
    use crate::item_name::{ITEM_NAMES, ItemName};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    // Can be used for stock, price lists, etc.
    pub struct InvStore(HashMap<ItemName, i32>);
    impl InvStore {
        pub fn new() -> Self {
            let mut store: InvStore = InvStore(HashMap::new());
            for item in ITEM_NAMES.iter() {
                store.0.insert(item.clone(), 0);
            }
            store
        }
        pub fn set(&mut self, item: ItemName, quantity: i32) {
            self.0.insert(item, quantity);
        }
        pub fn get(&self, item: &ItemName) -> Option<&i32> {
            self.0.get(item)
        }
        pub fn modify(&mut self, item: ItemName, delta: i32) {
            let entry = self.0.entry(item).or_insert(0);
            *entry += delta;
            if *entry < 0 {
                *entry = 0;
            }
        }
        pub fn items(&self) -> Vec<(&ItemName, &i32)> {
            self.0.iter().collect()
        }
        pub fn print(&self) {
            // Sort alphabetically by fname
            let mut items: Vec<(&ItemName, &i32)> = self.items();
            items.sort_by_key(|(item, _)| {
                let meta = ILM.get(item).unwrap();
                meta.fname.clone()
            });
            for (item, qty) in items {
                let meta = ILM.get(item).unwrap();
                println!("{:<24}: {}", meta.fname, qty);
            }
        }
    }
}

mod cargo_hold {
    use crate::inv_store::InvStore;
    use crate::item_meta::ILM;
    use crate::item_name::ItemName;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CargoHold {
        vol_max: i32,
        vol: i32,
        inv: InvStore,
    }
    impl CargoHold {
        pub fn new(vol_max: i32) -> Self {
            CargoHold {
                vol_max,
                vol: 0,
                inv: InvStore::new(),
            }
        }
        pub fn insert(&mut self, item: ItemName, quantity: i32) {
            self.inv.modify(item, quantity);
            self.calc_vol();
        }
        pub fn remove(&mut self, item: ItemName, quantity: i32) {
            self.inv.modify(item, -quantity);
            self.calc_vol();
        }
        pub fn print(&self) {
            println!("Cargo Hold: {}/{}", self.vol, self.vol_max);
            println!("-------");
            self.inv.print();
        }
        fn calc_vol(&mut self) {
            let mut total_vol = 0;
            for (item, qty) in self.inv.items() {
                if let Some(meta) = ILM.get(item) {
                    total_vol += meta.vol_pc * (*qty);
                }
            }
            self.vol = total_vol;
        }
    }
}

mod jump_drive {
    use crate::pos;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct JumpDrive {
        pub fuel_per_ly: i32,
        pub max_range: i32,
        pub fuel_max: i32,
        pub fuel_cur: i32,
    }
    impl JumpDrive {
        pub fn new(fuel_per_ly: i32, max_range: i32) -> Self {
            let fuel = fuel_per_ly * max_range * 8;
            JumpDrive {
                fuel_per_ly,
                max_range,
                fuel_max: fuel,
                fuel_cur: fuel,
            }
        }
        pub fn jump(&mut self, from: &pos::Position, to: &pos::Position) -> bool {
            let distance = from.distance(to);
            if distance > self.max_range {
                println!(
                    "Jump failed: distance {} exceeds max range {}",
                    distance, self.max_range
                );
                return false;
            }
            let fuel_needed = self.calc_fuel(distance);
            if fuel_needed > self.fuel_cur {
                println!(
                    "Jump failed: not enough fuel (need {}, have {})",
                    fuel_needed, self.fuel_cur
                );
                return false;
            }
            self.consume(fuel_needed);
            println!(
                "Jump successful: traveled {} light years, consumed {} fuel",
                distance, fuel_needed
            );
            true
        }
        pub fn print(&self) {
            println!("-------");
            println!("Jump Drive: {}/{}g", self.fuel_cur, self.fuel_max);
            println!("Max Range: {} ly", self.max_range);
            println!("Fuel per ly: {}", self.fuel_per_ly);
        }
        pub fn calc_fuel(&self, distance: i32) -> i32 {
            distance * self.fuel_per_ly
        }
        fn consume(&mut self, amount: i32) {
            self.fuel_cur -= amount;
            if self.fuel_cur < 0 {
                self.fuel_cur = 0;
            }
        }
    }
}

mod entity {
    use crate::cargo_hold::CargoHold;
    use crate::jump_drive::JumpDrive;
    use crate::pos;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EntityFlags {}
    impl EntityFlags {
        pub fn new() -> Self {
            EntityFlags {}
        }
    }

    #[derive(Hash, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub enum EntityClass {
        Station,
        Craft,
    }

    pub static ENTITY_CLASSES: [EntityClass; 2] = [EntityClass::Station, EntityClass::Craft];

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Entity {
        pub name: String,
        pub id: i32,
        pub class: EntityClass,
        pub pos: pos::Position,
        pub hold: CargoHold,
        pub jump_drive: JumpDrive,
        pub flags: EntityFlags,
        pub docked_to: Option<i32>,
    }
    impl Entity {
        pub fn new(name: &str) -> Self {
            Entity {
                name: name.to_string(),
                id: 0, // Set my list.add
                class: EntityClass::Craft,
                pos: pos::Position::new(0, 0),
                hold: CargoHold::new(1000),
                jump_drive: JumpDrive::new(10, 100),
                flags: EntityFlags::new(),
                docked_to: None,
            }
        }
        pub fn set_pos(&mut self, position: pos::Position) {
            self.pos = position;
        }
        pub fn jump_to(&mut self, destination: &pos::Position) -> bool {
            if self.jump_drive.jump(&self.pos, destination) {
                self.pos = destination.clone();
                true
            } else {
                false
            }
        }
    }
}

mod entity_maker {
    use crate::entity::Entity;
    use crate::entity::EntityClass;
    use crate::pos::Position;
    use crate::univ::UNIV;
    use rand::Rng;
    pub fn station(name_list: &Vec<String>) -> Entity {
        fn random_name(station_names: &Vec<String>) -> String {
            let mut rng = rand::rng();
            let index1 = rng.random_range(0..station_names.len());
            let index2 = rng.random_range(0..station_names.len());
            let index3 = rng.random_range(0..station_names.len());
            format!(
                "{}-{}-{}",
                station_names[index1], station_names[index2], station_names[index3]
            )
        }
        let mut ent = Entity::new(random_name(name_list).as_str());
        ent.class = EntityClass::Station;
        ent.set_pos(Position::random(UNIV.gal_size));
        ent
    }
}

mod entity_list {
    use crate::entity::Entity;
    use crate::entity_maker;
    use crate::pos::Position;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EntityList {
        id_acc: i32,
        entities: Vec<Entity>,
    }
    impl EntityList {
        pub fn new() -> Self {
            EntityList {
                entities: Vec::new(),
                id_acc: 0,
            }
        }
        pub fn add(&mut self, mut entity: Entity) {
            entity.id = self.id_acc;
            self.entities.push(entity);
        }
        pub fn get(&self, index: usize) -> Option<&Entity> {
            self.entities.get(index)
        }
        pub fn get_mut(&mut self, index: usize) -> Option<&mut Entity> {
            self.entities.get_mut(index)
        }
        pub fn get_by_id(&self, id: i32) -> Option<&Entity> {
            self.entities.iter().find(|ent| ent.id == id)
        }
        pub fn get_by_id_mut(&mut self, id: i32) -> Option<&mut Entity> {
            self.entities.iter_mut().find(|ent| ent.id == id)
        }
        pub fn generate_entities(&mut self, name_list: &Vec<String>, count: usize) {
            for i in 0..count {
                let ent = entity_maker::station(name_list);
                self.add(ent);
            }
        }
        pub fn get_player(&self) -> Option<&Entity> {
            self.get(0)
        }
        pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
            self.get_mut(0)
        }
        pub fn set_player(&mut self, ship: Entity) {
            if let Some(player) = self.get_mut(0) {
                *player = ship;
            }
        }
        pub fn list(&self) -> Vec<&Entity> {
            self.entities.iter().collect()
        }
        pub fn list_by_distance(&self, origin: Position, max_distance: i32) -> Vec<&Entity> {
            self.entities
                .iter()
                .filter(|ent| origin.distance(&ent.pos) <= max_distance)
                .collect()
        }
        pub fn print(&self, ship: &Entity) {
            for (i, ent) in self.entities.iter().enumerate() {
                let distance = ship.pos.distance(&ent.pos);
                println!(
                    "{}: {} [{}] ({} {})",
                    ent.id, ent.name, distance, ent.pos.x, ent.pos.y
                );
                println!("-------");
            }
        }
    }
}

mod input {
    use rustyline::Editor;

    pub fn prompt() -> String {
        // Create an Editor instance
        let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();

        // Load history from a file (ignore errors if file doesn't exist)
        let _ = rl.load_history("history.txt");

        // Read line with editing and history support
        match rl.readline("> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let _ = rl.save_history("history.txt");
                line.trim().to_string()
            }
            Err(_) => String::new(),
        }
    }
}

mod cmds {
    use crate::entity::Entity;
    pub fn status(ship: &Entity, ent_list: &EntityList) {
        println!("Ship Name: {}", ship.name);
        println!("Ship ID  : {}", ship.id);
        println!("Ship Class: {:?}", ship.class);
        let docked_ent = match ship.docked_to {
            Some(id) => ent_list.get_by_id(id).map(|e| e.name.clone()),
            None => None,
        };
        if let Some(docked_name) = docked_ent {
            println!(
                "Docked to: {} (ID {})",
                docked_name,
                ship.docked_to.unwrap()
            );
        } else {
            println!("Docked to: None");
        }
        ship.hold.print();
        ship.jump_drive.print();
        ship.pos.print();
    }

    pub fn jump_to(ent_list: &mut EntityList, ent_id: i32) {
        if let Some(target) = ent_list.get_by_id(ent_id) {
            // Clone/copy the needed data to end the immutable borrow
            let target_pos = target.pos.clone();
            let target_name = target.name.clone();

            let ship = ent_list.get_player_mut().unwrap();
            if ship.jump_to(&target_pos) {
                println!("Jumped to {}:", target_name);
                println!("Fuel remaining: {}g", ship.jump_drive.fuel_cur);
                ship.pos.print();
            } else {
                println!("Jump failed.");
            }
        } else {
            println!("No entity found with ID {}.", ent_id);
        }
    }

    use crate::pos::Position;
    pub fn jump_man(ship: &mut Entity, destination: Position) {
        if ship.jump_to(&destination) {
            println!("Jumped to ({}, {}):", destination.x, destination.y);
            println!("Fuel remaining: {}g", ship.jump_drive.fuel_cur);
            ship.pos.print();
        } else {
            println!("Jump failed.");
        }
    }

    pub fn jump_check(ship: &Entity, ent_list: &EntityList, ent_id: i32) {
        let ent = ent_list.get_by_id(ent_id);
        if let Some(target) = ent {
            let distance = ship.pos.distance(&target.pos);
            let fuel_needed = ship.jump_drive.calc_fuel(distance);
            println!("Jump Check to {}:", target.name);
            println!("Distance: {} ly", distance);
            println!("Fuel needed: {}g", fuel_needed);
            println!("Current fuel: {}g", ship.jump_drive.fuel_cur);
            println!(
                "Fuel after jump: {}g",
                ship.jump_drive.fuel_cur - fuel_needed
            );
        } else {
            println!("No entity found with ID {}.", ent_id);
        }
    }

    pub fn jump_check_man(ship: &Entity, destination: Position) {
        let distance = ship.pos.distance(&destination);
        let fuel_needed = ship.jump_drive.calc_fuel(distance);
        println!("Jump Check:");
        println!("Distance: {} ly", distance);
        println!("Fuel needed: {}g", fuel_needed);
        println!("Current fuel: {}g", ship.jump_drive.fuel_cur);
        println!(
            "Fuel after jump: {}g",
            ship.jump_drive.fuel_cur - fuel_needed
        );
    }

    use crate::entity::EntityClass;
    use crate::entity_list::EntityList;
    pub fn entities_list(list: &EntityList, ship: &Entity, max_distance: i32) {
        let mut found = 0;
        list.list_by_distance(ship.pos, max_distance)
            .iter()
            .for_each(|ent| {
                let class_str = match ent.class {
                    EntityClass::Station => "STAT",
                    EntityClass::Craft => "CRFT",
                };
                let distance = ship.pos.distance(&ent.pos);
                println!(
                    "{:<6}: [{}] {:<5} ly - ({:<5}, {:<5}) - {}",
                    ent.id, class_str, distance, ent.pos.x, ent.pos.y, ent.name
                );
                found += 1;
            });
        println!("Found {} entities within {} ly", found, max_distance);
    }

    pub fn cargo_list(ent: &Entity) {
        ent.hold.print();
    }

    pub fn dock_list(ship: &Entity, ent_list: &EntityList) {
        let nearby_stations: Vec<&Entity> = ent_list
            .list_by_distance(ship.pos, 1)
            .into_iter()
            .filter(|ent| ent.name.contains("Station"))
            .collect();
        if nearby_stations.is_empty() {
            println!("No stations nearby to dock with.");
        } else {
            println!("Nearby Stations:");
            for station in nearby_stations {
                let distance = ship.pos.distance(&station.pos);
                println!(
                    "{}: {} [{}] ({} {})",
                    station.id, station.name, distance, station.pos.x, station.pos.y
                );
            }
        }
    }

    pub fn dock(ent_list: &mut EntityList, ent_id: i32) {
        if let Some(target) = ent_list.get_by_id(ent_id) {
            // Clone/copy the needed data to end the immutable borrow
            let target_pos = target.pos.clone();
            let target_name = target.name.clone();

            let ship = ent_list.get_player_mut().unwrap();
            if ship.pos.distance(&target_pos) <= 1 {
                println!("Docked with {}.", target_name);
                ship.docked_to = Some(ent_id);
            } else {
                println!("Docking failed: not close enough to {}.", target_name);
            }
        } else {
            println!("No entity found with ID {}.", ent_id);
        }
    }

    pub fn name_ent(ent: &mut Entity, new_name: &str) {
        ent.name = new_name.to_string();
        println!("Renamed to {}", ent.name);
    }

    pub fn save(entities: &EntityList, filename: &str) {
        // Serialize entities to JSON and save to file
        let serialized = serde_json::to_string_pretty(&entities).unwrap();
        println!("Saving entities to {}...", filename);
        std::fs::write(filename, serialized).expect("Unable to write file");
    }

    pub fn load(filename: &str) -> EntityList {
        // Load entities from JSON file and deserialize
        println!("Loading entities from {}...", filename);
        let data = std::fs::read_to_string(filename).expect("Unable to read file");
        let entities: EntityList = serde_json::from_str(&data).unwrap();
        entities
    }
}

use crate::entity::Entity;
use crate::entity::EntityClass;
use crate::entity_list::EntityList;
use crate::input::prompt;
use crate::item_name::ItemName;
use crate::pos::Position;
use crate::univ::UNIV;
use std::io::{self, Write};

fn cmd_header(title: &str) {
    println!("{}", "▀".repeat(64));
    println!("██▀{:^57} ▄██", title.to_uppercase());
    println!("{}", "▄".repeat(64));
    println!();
}

fn main() {
    // Load names from file (names.txt)
    let mut name_list: Vec<String> = Vec::new();
    if let Ok(contents) = std::fs::read_to_string("names.txt") {
        for line in contents.lines() {
            name_list.push(line.to_string());
        }
    } else {
        // Exit
        println!("Error: Unable to read names.txt");
        return;
    }

    let mut entities = EntityList::new();

    let mut start_ship = Entity::new("My Ship");
    start_ship.class = EntityClass::Craft;
    start_ship.hold.insert(ItemName::MetalLow, 20);
    start_ship.hold.insert(ItemName::CompositeMid, 15);
    start_ship.hold.insert(ItemName::PolymerHigh, 5);
    let gal_center = UNIV.gal_size / 2;
    start_ship.set_pos(Position::new(gal_center, gal_center));
    entities.add(start_ship.clone());

    entities.generate_entities(&name_list, UNIV.starting_entities as usize);

    loop {
        let cmd_raw = prompt();
        let cmd: Vec<&str> = cmd_raw.split_whitespace().collect();
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        // Flush stdout
        io::stdout().flush().unwrap();
        match cmd[0] {
            "status" | "s" => {
                cmd_header("Ship Status");
                cmds::status(entities.get_player().unwrap(), &entities);
            }
            "jump" | "j" => {
                cmd_header("Jump");
                if cmd.len() < 2 {
                    println!("Usage: jump_man <entity_id>");
                    continue;
                }
                let ent_id: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entity ID.");
                        continue;
                    }
                };
                cmds::jump_to(&mut entities, ent_id);
            }
            "cargo" | "c" => {
                cmd_header("Cargo Hold");
                cmds::cargo_list(entities.get_player().unwrap());
            }
            "jump_man" | "jm" => {
                cmd_header("Jump (Manual)");
                if cmd.len() < 2 {
                    println!("Usage: jump_man <x> <y>");
                    continue;
                }
                let x: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid x coordinate.");
                        continue;
                    }
                };
                let y: i32 = match cmd[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid y coordinate.");
                        continue;
                    }
                };
                let destination = Position::new(x, y);
                cmds::jump_man(entities.get_player_mut().unwrap(), destination);
            }
            "jump_check" | "jc" => {
                cmd_header("Jump Check");
                if cmd.len() < 2 {
                    println!("Usage: jump_check <entity_id>");
                    continue;
                }
                let ent_id: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entity ID.");
                        continue;
                    }
                };
                cmds::jump_check(entities.get_player().unwrap(), &entities, ent_id);
            }
            "jump_check_man" | "jcm" => {
                cmd_header("Jump Check (Manual)");
                if cmd.len() < 3 {
                    println!("Usage: jump_check <x> <y>");
                    continue;
                }
                let x: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid x coordinate.");
                        continue;
                    }
                };
                let y: i32 = match cmd[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid y coordinate.");
                        continue;
                    }
                };
                let destination = Position::new(x, y);
                cmds::jump_check_man(entities.get_player().unwrap(), destination);
            }
            "entities" | "l" => {
                cmd_header("Entities List");
                let max_distance: i32;
                if cmd.len() < 2 {
                    max_distance = entities.get_player().unwrap().jump_drive.max_range;
                } else {
                    max_distance = match cmd[1].parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid max distance.");
                            continue;
                        }
                    };
                }
                cmds::entities_list(&entities, entities.get_player().unwrap(), max_distance);
            }
            "dock_list" | "dl" => {
                cmd_header("Dock");
                cmds::dock_list(entities.get_player().unwrap(), &entities);
            }
            "dock" | "d" => {
                cmd_header("Docking");
                if cmd.len() < 2 {
                    println!("Usage: dock <entity_id>");
                    continue;
                }
                let ent_id: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entity ID.");
                        continue;
                    }
                };
                cmds::dock(&mut entities, ent_id);
            }
            "name" => {
                cmd_header("Rename Ship");
                if cmd.len() < 2 {
                    println!("Usage: name <new_name>");
                    continue;
                }
                let new_name = cmd[1..].join(" ");
                cmds::name_ent(entities.get_player_mut().unwrap(), &new_name);
            }
            "save" => {
                cmd_header("Save Game");
                let filename = if cmd.len() < 2 {
                    "savegame.json"
                } else {
                    cmd[1]
                };
                cmds::save(&entities, filename);
            }
            "load" => {
                cmd_header("Load Game");
                let filename = if cmd.len() < 2 {
                    "savegame.json"
                } else {
                    cmd[1]
                };
                entities = cmds::load(filename);
                if let Some(ship) = entities.get_by_id_mut(0) {
                    // my_ship = ship;
                } else {
                    println!("Warning: No ship found in loaded data.");
                }
            }
            "quit" | "exit" | "q" => {
                cmd_header("Goodbye");
                println!("Exiting...");
                break;
            }
            _ => {
                cmd_header("ERROR");
                println!("Unknown command: {}", cmd_raw);
            }
        }
    }
}
