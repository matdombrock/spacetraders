#![allow(unused_variables)]
#![allow(dead_code)]

mod ch {
    pub static ARL: &str = "⮜";
    pub static ARR: &str = "⮞";
    pub static ARU: &str = "⮝";
    pub static ARD: &str = "⮟";
    pub static SP1: &str = "⏣";
}

mod fmt {
    pub fn credit(amount: &i32) -> String {
        format!("⧫{}", amount)
    }
    pub fn peice(amount: &i32) -> String {
        format!("{}pc", amount)
    }
    pub fn fuel(amount: &i32) -> String {
        format!("{}g", amount)
    }
    pub fn distance(amount: &i32) -> String {
        format!("{}ly", amount)
    }
    pub fn ent_id(ent_id: &i32) -> String {
        format!("#{}", ent_id)
    }
}

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
    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "X{} Y{}", self.x, self.y)
        }
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
        pub base_val: i32,
        pub rarity: i32, // 1-100
    }

    use std::collections::HashMap;
    pub struct InvListMeta(HashMap<ItemName, ItemMeta>);
    impl InvListMeta {
        pub fn new() -> Self {
            let mut map = HashMap::new();
            map.insert(
                ItemName::MetalLow,
                ItemMeta {
                    fname: "Metals (low grade)".to_string(),
                    vol_pc: 10,
                    base_val: 10,
                    rarity: 1,
                },
            );
            map.insert(
                ItemName::MetalMid,
                ItemMeta {
                    fname: "Metals (mid grade)".to_string(),
                    vol_pc: 20,
                    base_val: 20,
                    rarity: 2,
                },
            );
            map.insert(
                ItemName::MetalHigh,
                ItemMeta {
                    fname: "Metals (high grade)".to_string(),
                    vol_pc: 30,
                    base_val: 30,
                    rarity: 3,
                },
            );
            map.insert(
                ItemName::CompositeLow,
                ItemMeta {
                    fname: "Composites (low grade)".to_string(),
                    vol_pc: 15,
                    base_val: 25,
                    rarity: 10,
                },
            );
            map.insert(
                ItemName::CompositeMid,
                ItemMeta {
                    fname: "Composites (mid grade)".to_string(),
                    vol_pc: 25,
                    base_val: 35,
                    rarity: 20,
                },
            );
            map.insert(
                ItemName::CompositeHigh,
                ItemMeta {
                    fname: "Composites (high grade)".to_string(),
                    vol_pc: 35,
                    base_val: 45,
                    rarity: 30,
                },
            );
            map.insert(
                ItemName::PolymerLow,
                ItemMeta {
                    fname: "Polymers (low grade)".to_string(),
                    vol_pc: 12,
                    base_val: 20,
                    rarity: 10,
                },
            );
            map.insert(
                ItemName::PolymerMid,
                ItemMeta {
                    fname: "Polymers (mid grade)".to_string(),
                    vol_pc: 22,
                    base_val: 30,
                    rarity: 20,
                },
            );
            map.insert(
                ItemName::PolymerHigh,
                ItemMeta {
                    fname: "Polymers (high grade)".to_string(),
                    vol_pc: 32,
                    base_val: 40,
                    rarity: 30,
                },
            );
            InvListMeta(map)
        }
        pub fn get_by_enum(&self, item: &ItemName) -> Option<&ItemMeta> {
            self.0.get(item)
        }
        pub fn get_by_str(&self, item_str: &str) -> Option<&ItemMeta> {
            for (item_enum, meta) in self.0.iter() {
                if format!("{:?}", item_enum).to_lowercase() == item_str.to_lowercase() {
                    return Some(meta);
                }
            }
            None
        }
    }

    use std::sync::LazyLock;
    pub static ILM: LazyLock<InvListMeta> = LazyLock::new(|| InvListMeta::new());
}

mod inv_store {
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
    }
}

mod cargo_hold {
    use crate::inv_store::InvStore;
    use crate::item_meta::ILM;
    use crate::item_name::ItemName;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CargoHold {
        pub vol_max: i32,
        pub vol: i32,
        pub inv: InvStore,
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
        fn calc_vol(&mut self) {
            let mut total_vol = 0;
            for (item, qty) in self.inv.items() {
                if let Some(meta) = ILM.get_by_enum(item) {
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
    pub struct JumpRes {
        pub success: bool,
        pub message: String,
        pub distance: i32,
        pub fuel_used: i32,
    }
    impl JumpRes {
        pub fn new() -> Self {
            JumpRes {
                success: false,
                message: String::new(),
                distance: 0,
                fuel_used: 0,
            }
        }
    }

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
        pub fn jump(&mut self, from: &pos::Position, to: &pos::Position) -> JumpRes {
            let mut res = JumpRes::new();
            let distance = from.distance(to);
            if distance > self.max_range {
                res.message = format!(
                    "Jump failed: distance {} exceeds max range {}",
                    distance, self.max_range
                );
                return res;
            }
            let fuel_needed = self.calc_fuel(distance);
            if fuel_needed > self.fuel_cur {
                res.message = format!(
                    "Jump failed: not enough fuel (need {}, have {})",
                    fuel_needed, self.fuel_cur
                );
                return res;
            }
            self.consume(fuel_needed);
            res.message = format!(
                "Jump successful: traveled {} light years, consumed {} fuel",
                distance, fuel_needed
            );
            res.success = true;
            res.distance = distance;
            res.fuel_used = fuel_needed;
            res
        }
        pub fn refuel_amt(&self) -> i32 {
            self.fuel_max - self.fuel_cur
        }
        pub fn refuel(&mut self, amount: i32) {
            self.fuel_cur += amount;
            if self.fuel_cur > self.fuel_max {
                self.fuel_cur = self.fuel_max;
            }
        }
        pub fn print(&self) {
            println!("Jump Drive: {}/{}g", self.fuel_cur, self.fuel_max);
            println!("Max Range: {} ly", self.max_range);
            println!("Fuel per ly: {}", self.fuel_per_ly);
        }
        pub fn calc_fuel(&self, distance: i32) -> i32 {
            distance * self.fuel_per_ly
        }
        pub fn fuel_str(&self) -> String {
            format!("{}/{} g", self.fuel_cur, self.fuel_max)
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
    use crate::inv_store::InvStore;
    use crate::jump_drive::JumpDrive;
    use crate::jump_drive::JumpRes;
    use crate::pos;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EntityFlags {
        pub has_dock: bool,
    }
    impl EntityFlags {
        pub fn new() -> Self {
            EntityFlags { has_dock: false }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EntityFinance {
        pub credits: i32,
        pub prices: InvStore,
        pub profit_margin: i32, // Percentage
    }
    impl EntityFinance {
        pub fn new() -> Self {
            EntityFinance {
                credits: 10000,
                prices: InvStore::new(),
                profit_margin: 20,
            }
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
        pub fin: EntityFinance,
        pub hold: CargoHold,
        pub jump_drive: JumpDrive,
        pub flags: EntityFlags,
        pub docked_id: Option<i32>,
        pub targeting_id: Option<i32>,
    }
    impl Entity {
        pub fn new(name: &str) -> Self {
            Entity {
                name: name.to_string(),
                id: 0, // Set my list.add
                class: EntityClass::Craft,
                pos: pos::Position::new(0, 0),
                fin: EntityFinance::new(),
                hold: CargoHold::new(1000),
                jump_drive: JumpDrive::new(10, 100),
                flags: EntityFlags::new(),
                docked_id: None,
                targeting_id: None,
            }
        }
        pub fn set_pos(&mut self, position: pos::Position) {
            self.pos = position;
        }
        pub fn jump(&mut self, destination: &pos::Position) -> JumpRes {
            // Check if docked
            let mut res = JumpRes::new();
            if self.docked_id.is_some() {
                res.success = false;
                res.message = "Cannot jump while docked.".to_string();
                return res;
            }
            res = self.jump_drive.jump(&self.pos, destination);
            if res.success {
                self.pos = destination.clone();
                res.success = true;
                res
            } else {
                res.success = false;
                res
            }
        }
    }
}

mod entity_maker {
    use crate::entity::Entity;
    use crate::entity::EntityClass;
    use crate::inv_store::InvStore;
    use crate::item_meta::ILM;
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
        fn random_prices() -> InvStore {
            let mut rng = rand::rng();
            // We need a mutable and immutable InvStore
            let list = InvStore::new();
            let mut prices = InvStore::new();
            for (item, qty) in list.items() {
                if let Some(meta) = ILM.get_by_enum(item) {
                    let price = meta.base_val + rng.random_range(-5..15);
                    prices.set(item.clone(), price);
                }
            }
            prices
        }
        let mut ent = Entity::new(random_name(name_list).as_str());
        ent.fin.prices = random_prices();
        ent.class = EntityClass::Station;
        ent.flags.has_dock = true;
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
            self.id_acc += 1;
        }
        pub fn get(&self, index: i32) -> Option<&Entity> {
            self.entities.get(index as usize)
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
            let list = self
                .entities
                .iter()
                .filter(|ent| origin.distance(&ent.pos) <= max_distance)
                .collect();
            // Sort by distance
            let mut sorted_list: Vec<&Entity> = list;
            sorted_list.sort_by_key(|ent| origin.distance(&ent.pos));
            sorted_list
        }
        pub fn print(&self, ship: &Entity) {
            for (i, ent) in self.entities.iter().enumerate() {
                let distance = ship.pos.distance(&ent.pos);
                println!(
                    "{}: {} [{}] ({} {})",
                    ent.id, ent.name, distance, ent.pos.x, ent.pos.y
                );
            }
        }
    }
}

mod input {
    use crate::ch;
    use colored::*;
    use rustyline::Editor;

    pub fn prompt() -> String {
        // Create an Editor instance
        let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().unwrap();

        // Load history from a file (ignore errors if file doesn't exist)
        let _ = rl.load_history("history.txt");

        let prompt_str = format!("{} | ", ch::SP1).bright_green().to_string();

        match rl.readline(&prompt_str) {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let _ = rl.save_history("history.txt");
                line.trim().to_string()
            }
            Err(_) => String::new(),
        }
    }
}

// Actions represent things the player can do
// They should never print anything
// Formatting and UX is up to ui functions
// Galaxy Manager
mod gm {

    use crate::entity::Entity;
    use crate::entity_list::EntityList;
    use crate::jump_drive::JumpRes;
    use crate::pos::Position;

    pub struct GMRes {
        pub success: bool,
    }

    pub struct GMResMsg {
        pub success: bool,
        pub message: String,
    }

    pub struct GMResJumpCheck {
        pub distance: i32,
        pub fuel_needed: i32,
        pub fuel_cur: i32,
        pub fuel_after: i32,
        pub can_jump: bool,
    }

    pub type GMResJump = JumpRes;
    pub struct GMResEntList {
        pub entities: Vec<Entity>,
    }

    pub struct GM {
        pub tick: i32,
    }
    impl GM {
        pub fn new() -> Self {
            GM { tick: 0 }
        }
        pub fn set_target(&self, player: &mut Entity, ent_id: i32) -> GMRes {
            player.targeting_id = Some(ent_id);
            GMRes { success: true }
        }

        pub fn jump_check(&self, player: &Entity, target: &Position) -> GMResJumpCheck {
            let distance = player.pos.distance(target);
            let fuel_needed = player.jump_drive.calc_fuel(distance);
            let fuel_cur = player.jump_drive.fuel_cur;
            let fuel_after = fuel_cur - fuel_needed;
            let can_jump = distance <= player.jump_drive.max_range && fuel_needed <= fuel_cur;
            GMResJumpCheck {
                distance,
                fuel_needed,
                fuel_cur,
                fuel_after,
                can_jump,
            }
        }

        pub fn jump(&mut self, player: &mut Entity, destination: &Position) -> GMResJump {
            self.tick += 1;
            player.jump(&destination)
        }

        pub fn dock_list(&self, player: &Entity, ent_list: &EntityList) -> GMResEntList {
            let nearby_stations: Vec<&Entity> = ent_list
                .list_by_distance(player.pos, 1)
                .into_iter()
                .filter(|ent| ent.flags.has_dock)
                .collect();
            GMResEntList {
                entities: nearby_stations.into_iter().cloned().collect(),
            }
        }

        pub fn dock(&self, ent_list: &mut EntityList, ent_id: i32) -> GMResMsg {
            let mut res = GMResMsg {
                success: false,
                message: String::new(),
            };
            if let Some(target) = ent_list.get_by_id(ent_id) {
                if !target.flags.has_dock {
                    res.message =
                        format!("Entity ID {} does not have docking capabilities.", ent_id);
                    return res;
                }
                // Clone/copy the needed data to end the immutable borrow
                let target_pos = target.pos.clone();
                let target_name = target.name.clone();

                let ship = ent_list.get_player_mut().unwrap();
                if ship.pos.distance(&target_pos) <= 1 {
                    res.message = format!("Docked with {}.", target_name);
                    res.success = true;
                    ship.docked_id = Some(ent_id);
                } else {
                    res.message = format!("Docking failed: not close enough to {}.", target_name);
                }
            } else {
                res.message = format!("No entity found with ID {}.", ent_id);
            }
            res
        }

        pub fn undock(&self, player: &mut Entity) -> GMResMsg {
            let mut res = GMResMsg {
                success: false,
                message: String::new(),
            };
            if let Some(docked_id) = player.docked_id {
                res.message = format!("Undocked from entity ID {}.", docked_id);
                player.docked_id = None;
            } else {
                res.message = format!("Not currently docked to any entity.");
            }
            res
        }

        pub fn name_ent(&self, ent: &mut Entity, new_name: &str) -> GMResMsg {
            ent.name = new_name.to_string();
            GMResMsg {
                success: true,
                message: format!("Renamed to {}", ent.name),
            }
        }

        pub fn save(&self, entities: &EntityList, filename: &str) -> GMResMsg {
            // Serialize entities to JSON and save to file
            let serialized = serde_json::to_string_pretty(&entities).unwrap();
            std::fs::write(filename, serialized).expect("Unable to write file");
            GMResMsg {
                success: true,
                message: format!("Saved to {}", filename),
            }
        }

        pub fn load(&self, entities: &mut EntityList, filename: &str) -> GMResMsg {
            // Load entities from JSON file and deserialize
            let data = std::fs::read_to_string(filename).expect("Unable to read file");
            *entities = serde_json::from_str(&data).unwrap();
            GMResMsg {
                success: true,
                message: "loaded".to_string(),
            }
        }
    }
}

// CLI functions call actions::
// Handle IO
mod cli {
    use crate::entity_list::EntityList;
    use crate::gm::GM;
    use crate::item_meta::ILM;
    use crate::pos::Position;
    use crate::{
        ItemName,
        entity::{Entity, EntityClass},
    };
    use crate::{ch, fmt};
    use colored::*;
    use std::collections::HashMap;

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum CmdName {
        Help,
        Target,
        Scan,
        Jump,
        JumpMan,
        JumpCheckMan,
        JumpCheck,
        JumpRel,
        JumpCheckRel,
        Entities,
        Cargo,
        Buy,
        Sell,
        DockList,
        Dock,
        Undock,
        Refuel,
        Name,
        Time,
        Save,
        Load,
        Quit,
    }
    pub struct CmdMeta {
        full: &'static str,
        short: &'static str,
        params: &'static str, // req opt?
        desc: &'static str,
    }
    type CmdMetaMap = HashMap<CmdName, CmdMeta>;
    fn make_cli_meta() -> CmdMetaMap {
        let mut map: CmdMetaMap = HashMap::new();
        map.insert(
            CmdName::Help,
            CmdMeta {
                full: "help",
                short: "h",
                params: "",
                desc: "Show this help message.",
            },
        );
        map.insert(
            CmdName::Target,
            CmdMeta {
                full: "target",
                short: "t",
                params: "ent_id?",
                desc: "Set or view current target entity by ID.",
            },
        );
        map.insert(
            CmdName::Scan,
            CmdMeta {
                full: "scan",
                short: "s",
                params: "ent_id?",
                desc: "Scan an entity by ID or self if no ID provided.",
            },
        );
        map.insert(
            CmdName::Jump,
            CmdMeta {
                full: "jump",
                short: "j",
                params: "ent_id",
                desc: "Jump to target entity by ID.",
            },
        );
        map.insert(
            CmdName::JumpMan,
            CmdMeta {
                full: "jump_man",
                short: "jm",
                params: "x y",
                desc: "Jump to specified coordinates.",
            },
        );
        map.insert(
            CmdName::JumpCheckMan,
            CmdMeta {
                full: "jump_check_man",
                short: "jcm",
                params: "x y",
                desc: "Check jump feasibility to specified coordinates.",
            },
        );
        map.insert(
            CmdName::JumpCheck,
            CmdMeta {
                full: "jump_check",
                short: "jc",
                params: "ent_id",
                desc: "Check jump feasibility to target entity by ID.",
            },
        );
        map.insert(
            CmdName::JumpRel,
            CmdMeta {
                full: "jump_rel",
                short: "jr",
                params: "x y",
                desc: "Jump to coordinates relative to current position.",
            },
        );
        map.insert(
            CmdName::JumpCheckRel,
            CmdMeta {
                full: "jump_check_rel",
                short: "jcr",
                params: "x y",
                desc: "Check jump feasibility to coordinates relative to current position.",
            },
        );
        map.insert(
            CmdName::Entities,
            CmdMeta {
                full: "entities",
                short: "l",
                params: "dist",
                desc: "List entities within jump range.",
            },
        );
        map.insert(
            CmdName::Cargo,
            CmdMeta {
                full: "cargo",
                short: "c",
                params: "ent_id?",
                desc: "View cargo hold contents.",
            },
        );
        map.insert(
            CmdName::Buy,
            CmdMeta {
                full: "buy",
                short: "b",
                params: "ent_id item qty",
                desc: "Buy specified quantity of an item.",
            },
        );
        map.insert(
            CmdName::Sell,
            CmdMeta {
                full: "sell",
                short: "sl",
                params: "ent_id item qty",
                desc: "Sell specified quantity of an item.",
            },
        );
        map.insert(
            CmdName::DockList,
            CmdMeta {
                full: "dock_list",
                short: "dl",
                params: "",
                desc: "List nearby docking-capable entities.",
            },
        );
        map.insert(
            CmdName::Dock,
            CmdMeta {
                full: "dock",
                short: "d",
                params: "ent_id",
                desc: "Dock with a specified entity by ID.",
            },
        );
        map.insert(
            CmdName::Undock,
            CmdMeta {
                full: "undock",
                short: "ud",
                params: "",
                desc: "Undock from the currently docked entity.",
            },
        );
        map.insert(
            CmdName::Refuel,
            CmdMeta {
                full: "refuel",
                short: "rf",
                params: "",
                desc: "Refuel the ship's jump drive while docked.",
            },
        );
        map.insert(
            CmdName::Name,
            CmdMeta {
                full: "name",
                short: "n",
                params: "",
                desc: "Rename the player's ship.",
            },
        );
        map.insert(
            CmdName::Time,
            CmdMeta {
                full: "time",
                short: "ti",
                params: "",
                desc: "Show the current game time (tick).",
            },
        );
        map.insert(
            CmdName::Save,
            CmdMeta {
                full: "save",
                short: "sv",
                params: "file",
                desc: "Save the current game state to a file.",
            },
        );
        map.insert(
            CmdName::Load,
            CmdMeta {
                full: "load",
                short: "ld",
                params: "file",
                desc: "Load a game state from a file.",
            },
        );
        map.insert(
            CmdName::Quit,
            CmdMeta {
                full: "quit",
                short: "q",
                params: "",
                desc: "Exit the game.",
            },
        );
        map
    }
    pub struct CLI {
        pub last_id: i32,
        pub meta: CmdMetaMap,
        // TODO:
        // Should gm be owned by main and passed to cli?
        // This would make it easier to share with TUI
        pub gm: GM,
    }
    impl CLI {
        // Meta

        pub fn new() -> Self {
            CLI {
                last_id: 0,
                meta: make_cli_meta(),
                gm: GM::new(),
            }
        }

        pub fn check_cmd(&self, v: &str, target: CmdName) -> bool {
            let cmd = self.meta.get(&target).unwrap();
            v == cmd.full || v == cmd.short
        }

        // Generate a single line entity string
        fn ent_line_str(&self, ent_id: i32, ent_list: &EntityList) -> String {
            let ent = ent_list.get_by_id(ent_id).unwrap();
            format!("{} [{}] ({} {})", ent.name, ent.id, ent.pos.x, ent.pos.y,)
        }
        // Print single line entity string
        fn print_ent_line(&self, ent_id: i32, ent_list: &EntityList) {
            let ent_str = self.ent_line_str(ent_id, ent_list);
            println!("{}{:^62}{}", ch::ARL, ent_str, ch::ARR);
        }

        // Commands

        pub fn intro(&self) {
            CLI::cli_header("SpaceTrade.rs CLI");
            println!("Type 'help' for a list of commands.");
        }

        pub fn help(&self, cmd: Vec<&str>) {
            fn print_full(cmd: &CmdMeta) {
                print!("{} {}", cmd.full.green(), cmd.params.yellow());
                println!(" | {}", cmd.short.green());
                println!("-- {}", cmd.desc);
            }
            CLI::cli_header("Help");
            let mut mode = "short";
            if cmd.len() >= 2 {
                mode = cmd[1];
            }
            if mode == "full" {
                println!("Available commands:");
                let mut cmds: Vec<(&CmdName, &CmdMeta)> = self.meta.iter().collect();
                cmds.sort_by_key(|(_, cmd)| cmd.full);
                for (_, cmd) in cmds {
                    print_full(cmd);
                }
                return;
            }
            if mode == "min" {
                println!("Available commands:");
                let mut cmds: Vec<(&CmdName, &CmdMeta)> = self.meta.iter().collect();
                cmds.sort_by_key(|(_, cmd)| cmd.full);
                for (_, cmd) in cmds {
                    print!("{} ", cmd.full);
                }
                println!();
                return;
            }
            if mode == "short" {
                println!("Available commands:");
                let mut cmds: Vec<(&CmdName, &CmdMeta)> = self.meta.iter().collect();
                cmds.sort_by_key(|(_, cmd)| cmd.full);
                for (_, cmd) in cmds {
                    println!("{} ", cmd.full);
                }
                return;
            }
            // Check if mode matches a command
            // If so, print detailed help for that command
            // Otherwise, print error
            for (_, cmd) in self.meta.iter() {
                if mode == cmd.full || mode == cmd.short {
                    print_full(cmd);
                    return;
                }
            }
            println!("No help found for '{}'.", mode);
        }

        pub fn target(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Target");
            let ship = entities.get_player_mut().unwrap();
            if cmd.len() < 2 {
                if let Some(target_id) = ship.targeting_id {
                    println!("Current target ID: {}", target_id);
                } else {
                    println!("No target set.");
                }
                return;
            }
            let ent_id: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entity ID.");
                    return;
                }
            };
            self.gm.set_target(ship, ent_id);

            self.print_ent_line(ent_id, entities);
            println!("Target set to entity ID {}", ent_id);

            self.last_id = ent_id;
        }

        pub fn scan(&mut self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Scan Report");
            let mut scan_target = entities.get_player().unwrap();
            if cmd.len() == 1 {
                scan_target = entities.get_player().unwrap();
            } else if cmd.len() == 2 {
                let scan_target_id: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entity ID.");
                        return;
                    }
                };
                if let Some(_ent) = entities.get_by_id(scan_target_id) {
                    scan_target = _ent;
                } else {
                    println!("No entity found with ID {}.", scan_target_id);
                }
            } else {
                println!("Usage: scan [<entity_id>]");
            }

            self.print_ent_line(scan_target.id, entities);

            // Get the current target of the scan target
            let targeting_id = match scan_target.targeting_id {
                Some(id) => id,
                None => -1,
            };
            let targeting_str = if targeting_id == -1 {
                "None".to_string()
            } else {
                fmt::ent_id(&targeting_id)
            };

            // Check if the scan target is docked
            let docked_id = match scan_target.docked_id {
                Some(id) => id,
                none => -1,
            };
            let docked_str = if docked_id == -1 {
                "None".to_string()
            } else {
                fmt::ent_id(&docked_id)
            };

            println!("{:<12}: {}", "Name", scan_target.name);
            println!("{:<12}: {:?}", "Class", scan_target.class);
            println!(
                "{:<12}: {}",
                "Credits",
                fmt::credit(&scan_target.fin.credits)
            );
            println!("{:<12}: {}", "Targeting", targeting_str);
            println!("{:<12}: {}", "Docked to", docked_str);

            println!("{:<12}: {}", "Position", scan_target.pos);

            self.last_id = scan_target.id;
        }

        pub fn jump(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Jump");
            if cmd.len() < 2 {
                println!("Usage: jump <entity_id>");
                return;
            }
            let ent_id: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entity ID.");
                    return;
                }
            };
            let ent_pos = if let Some(target) = entities.get_by_id(ent_id) {
                target.pos.clone()
            } else {
                println!("No entity found with ID {}.", ent_id);
                return;
            };

            self.print_ent_line(ent_id, entities);

            self._jump(entities.get_player_mut().unwrap(), &ent_pos);

            self.last_id = ent_id;
        }

        pub fn jump_check(&mut self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Jump Check");
            if cmd.len() < 2 {
                println!("Usage: jump_check <entity_id>");
                return;
            }
            let ent_id: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entity ID.");
                    return;
                }
            };
            let target_pos = if let Some(target) = entities.get_by_id(ent_id) {
                target.pos.clone()
            } else {
                println!("No entity found with ID {}.", ent_id);
                return;
            };
            let ship = entities.get_player().unwrap();
            let res = self.gm.jump_check(ship, &target_pos);
            self.print_ent_line(ent_id, entities);
            println!("Jump Check complete.");
            println!("Distance: {} ly", res.distance);
            println!("Fuel needed: {}g", res.fuel_needed);
            println!("Current fuel: {}g", res.fuel_cur);
            println!("Fuel after jump: {}g", res.fuel_after);
            if res.can_jump {
                println!("Jump is possible.");
            } else {
                println!("Jump is NOT possible.");
            }

            self.last_id = ent_id;
        }

        pub fn jump_man(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Jump (Manual)");
            if cmd.len() < 2 {
                println!("Usage: jump_man <x> <y>");
                return;
            }
            let x: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid x coordinate.");
                    return;
                }
            };
            let y: i32 = match cmd[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid y coordinate.");
                    return;
                }
            };
            let destination = Position::new(x, y);
            self._jump(entities.get_player_mut().unwrap(), &destination);
        }

        pub fn jump_check_man(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Jump Check (Manual)");
            if cmd.len() < 3 {
                println!("Usage: jump_check <x> <y>");
                return;
            }
            let x: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid x coordinate.");
                    return;
                }
            };
            let y: i32 = match cmd[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid y coordinate.");
                    return;
                }
            };
            let destination = Position::new(x, y);
            self.gm
                .jump_check(entities.get_player().unwrap(), &destination);
        }

        pub fn jump_rel(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Jump (Relative)");
            if cmd.len() < 3 {
                println!("Usage: jump_rel <dx> <dy>");
                return;
            }
            let dx: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid x delta.");
                    return;
                }
            };
            let dy: i32 = match cmd[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid y delta.");
                    return;
                }
            };
            let ship = entities.get_player_mut().unwrap();
            let destination = Position::new(ship.pos.x + dx, ship.pos.y + dy);
            self._jump(ship, &destination);
        }

        pub fn jump_check_rel(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Jump Check (Relative)");
            if cmd.len() < 3 {
                println!("Usage: jump_check_rel <dx> <dy>");
                return;
            }
            let dx: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid x delta.");
                    return;
                }
            };
            let dy: i32 = match cmd[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid y delta.");
                    return;
                }
            };
            let ship = entities.get_player().unwrap();
            let destination = Position::new(ship.pos.x + dx, ship.pos.y + dy);
            self.gm.jump_check(ship, &destination);
        }

        pub fn cargo(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Cargo Hold");

            // Default to player entity
            let mut ent: &Entity = entities.get_player().unwrap();

            // Override with specified entity ID
            if cmd.len() > 1 {
                let ent_id: i32 = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid entity ID.");
                        return;
                    }
                };
                if let Some(_ent) = entities.get_by_id(ent_id) {
                    ent = _ent;
                } else {
                    println!("No entity found with ID {}.", ent_id);
                }
            }

            self.print_ent_line(ent.id, entities);

            let hold = &ent.hold;
            println!("Cargo Hold: {}/{}", hold.vol, hold.vol_max);
            println!("-------");

            // Sort alphabetically by fname
            let mut items: Vec<(&ItemName, &i32)> = hold.inv.items();
            items.sort_by_key(|(item, _)| {
                let meta = ILM.get_by_enum(item).unwrap();
                meta.fname.clone()
            });
            let prices = &ent.fin.prices;
            for (item, qty) in items {
                let meta = ILM.get_by_enum(item).unwrap();
                let price = prices.get(item).unwrap_or(&0);
                println!(
                    "{:<24}: {} - {}",
                    meta.fname,
                    fmt::peice(qty),
                    fmt::credit(price)
                );
            }
        }

        pub fn buy(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Buy Items");
            if cmd.len() < 4 {
                println!("Usage: buy <ent_id> <item> <qty>");
                return;
            }
            let ent_id: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entity ID.");
                    return;
                }
            };
            let item = match ILM.get_by_str(cmd[2]) {
                Some(it) => it,
                None => {
                    println!("Invalid item name: '{}'", cmd[2]);
                    return;
                }
            };
            let qty: i32 = match cmd[3].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid quantity.");
                    return;
                }
            };
            // Check distance from player to ent, must be within 1 ly
            let player = entities.get_player().unwrap();
            let ent = entities.get(ent_id).unwrap();
            let distance = player.pos.distance(&ent.pos);
            if distance > 0 {
                println!("Too far to trade!");
                return;
            }
            // Station require docking to trade
            if ent.class == EntityClass::Station && player.docked_id != Some(ent.id) {
                println!("Must be docked to trade with a station");
                return;
            }
            println!(
                "Bought {}pc of {} from entity ID {}",
                qty, item.fname, ent_id
            );
        }

        pub fn sell(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Sell Items");
        }

        pub fn entities(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Entities List");
            let max_distance: i32;
            if cmd.len() < 2 {
                max_distance = entities.get_player().unwrap().jump_drive.max_range;
            } else {
                max_distance = match cmd[1].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid max distance.");
                        return;
                    }
                };
            }
            let mut found = 0;
            let ent = entities.get_player().unwrap();
            entities
                .list_by_distance(ent.pos, max_distance)
                .iter()
                .for_each(|target| {
                    let class_str = match target.class {
                        EntityClass::Station => "STAT",
                        EntityClass::Craft => "CRFT",
                    };
                    let distance = ent.pos.distance(&target.pos);
                    if distance == 0 {
                        print!("^^ ");
                    } else {
                        print!("<< ");
                    }
                    println!(
                        "{:<6}: [{}] {:>5} ly - ({:>5}, {:>5}) - {}",
                        target.id, class_str, distance, target.pos.x, target.pos.y, target.name
                    );
                    found += 1;
                });
            println!("Found {} entities within {} ly", found, max_distance);
        }

        pub fn dock_list(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Dock List");
            println!("Nearby docking-capable entities:");
            let res = self.gm.dock_list(entities.get_player().unwrap(), &entities);
            if res.entities.is_empty() {
                println!("No docking-capable entities nearby.");
            } else {
                for ent in res.entities {
                    let distance = entities.get_player().unwrap().pos.distance(&ent.pos);
                    println!("ID {}: {} ({} ly away)", ent.id, ent.name, distance);
                }
            }
        }

        pub fn dock(&mut self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Docking");
            if cmd.len() < 2 {
                println!("Usage: dock <entity_id>");
                return;
            }
            let ent_id: i32 = match cmd[1].parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid entity ID.");
                    return;
                }
            };

            self.print_ent_line(ent_id, entities);

            let res = self.gm.dock(entities, ent_id);
            if res.success {
                println!("Docked to: {}", ent_id);
            } else {
                println!("Docking failed: {}", res.message);
            }

            self.last_id = ent_id;
        }

        pub fn undock(&self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Undocking");
            self.gm.undock(entities.get_player_mut().unwrap());
        }

        pub fn name(&self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Rename Ship");
            if cmd.len() < 2 {
                println!("Usage: name <new_name>");
                return;
            }
            let new_name = cmd[1..].join(" ");
            self.gm
                .name_ent(entities.get_player_mut().unwrap(), &new_name);
        }

        pub fn time(&self, cmd: Vec<&str>) {
            CLI::cli_header("Game Time");
            println!("Current game tick: {}", self.gm.tick);
        }

        pub fn refuel(&self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Refuel Ship");
            let ship = entities.get_player_mut().unwrap();
            // Check if docked
            if ship.docked_id.is_none() {
                println!("Must be docked to refuel.");
                return;
            }
            let mut amt_needed = ship.jump_drive.refuel_amt();
            if amt_needed == 0 {
                println!("Jump drive is already full.");
                return;
            }
            let cost_per_g = 0.2; // Example cost
            let mut total_cost = (amt_needed as f32 * cost_per_g) as i32;
            if ship.fin.credits < total_cost {
                println!(
                    "Not enough credits to refuel completely. Need {}, have {}.",
                    total_cost, ship.fin.credits
                );
                amt_needed = (ship.fin.credits as f32 / cost_per_g) as i32;
                println!("You can only afford to refuel {} g.", amt_needed);
                total_cost = ship.fin.credits;
            }
            ship.fin.credits -= total_cost;
            ship.jump_drive.refuel(amt_needed);
            println!(
                "Refueled {} g for {} credits. Current fuel: {}",
                amt_needed,
                total_cost,
                ship.jump_drive.fuel_str()
            );
        }

        pub fn save(&self, cmd: Vec<&str>, entities: &EntityList) {
            CLI::cli_header("Save Game");
            let filename = if cmd.len() < 2 {
                "savegame.json"
            } else {
                cmd[1]
            };
            self.gm.save(&entities, filename);
            println!("Game saved to {}", filename);
        }

        pub fn load(&self, cmd: Vec<&str>, entities: &mut EntityList) {
            CLI::cli_header("Load Game");
            let filename = if cmd.len() < 2 {
                "savegame.json"
            } else {
                cmd[1]
            };
            self.gm.load(entities, filename);
            println!("Loaded game from {}", filename);
        }

        pub fn quit(&self, cmd: Vec<&str>) {
            CLI::cli_header("Goodbye");
            println!("Exiting...");
        }

        pub fn unknown(&self, cmd: Vec<&str>) {
            CLI::cli_header("Unknown Command");
            println!("Unknown command: {}", cmd[0]);
        }

        fn _jump(&mut self, ent: &mut Entity, target: &Position) {
            println!("Attempting jump to {}", target);
            let res = self.gm.jump(ent, target);
            if res.success {
                println!("Jump successful.");
                println!("Distance traveled: {} ly", res.distance);
                println!("Fuel used: {} g", res.fuel_used);
                println!("Current Fuel: {}", ent.jump_drive.fuel_str());
                println!("New Position: {}", ent.pos);
            } else {
                println!("Jump failed: {}", res.message);
            }
        }
        fn cli_header(title: &str) {
            println!("{}", "▀".repeat(64).green());
            println!(
                "██▀{:^67}▄██",
                format!("⏣ |⯟ |- {} -|⯟ |⏣", title.to_uppercase().bright_green())
            );
            println!("{}", "▄".repeat(64).green());
            println!();
        }
    }
}

use crate::cli::CmdName;
use crate::entity::Entity;
use crate::entity::EntityClass;
use crate::entity_list::EntityList;
use crate::input::prompt;
use crate::item_name::ItemName;
use crate::pos::Position;
use crate::univ::UNIV;
use std::io::{self, Write};

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

    let mut start_ship = Entity::new("Ferris 1");
    start_ship.class = EntityClass::Craft;
    start_ship.hold.insert(ItemName::MetalLow, 20);
    start_ship.hold.insert(ItemName::CompositeMid, 15);
    start_ship.hold.insert(ItemName::PolymerHigh, 5);
    let gal_center = UNIV.gal_size / 2;
    start_ship.set_pos(Position::new(gal_center, gal_center));
    entities.add(start_ship.clone());

    entities.generate_entities(&name_list, UNIV.starting_entities as usize);

    let mut cli = cli::CLI::new();

    cli.intro();

    loop {
        println!();
        let mut cmd_raw = prompt();
        // Replace "@" with current target ID
        if let Some(target_id) = entities.get_player().unwrap().targeting_id {
            cmd_raw = cmd_raw.replace("@", &target_id.to_string());
        }
        // Replace # with last used ID
        if cmd_raw == "#" {
            println!("Last ID: {}", cli.last_id);
            continue;
        }
        cmd_raw = cmd_raw.replace("#", &cli.last_id.to_string());

        // Split command into parts
        let cmd: Vec<&str> = cmd_raw.split_whitespace().collect();
        if cmd.is_empty() {
            continue;
        }

        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        // Flush stdout
        io::stdout().flush().unwrap();

        // TODO:
        // Use a command map to call functions dynamically
        // This match is no good
        match cmd[0] {
            v if cli.check_cmd(v, CmdName::Help) => {
                cli.help(cmd);
            }
            v if cli.check_cmd(v, CmdName::Target) => {
                cli.target(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Scan) => {
                cli.scan(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Jump) => {
                cli.jump(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::JumpMan) => {
                cli.jump_man(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::JumpCheck) => {
                cli.jump_check(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::JumpCheckMan) => {
                cli.jump_check_man(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::JumpRel) => {
                cli.jump_rel(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::JumpCheckRel) => {
                cli.jump_check_rel(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Entities) => {
                cli.entities(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Cargo) => {
                cli.cargo(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Buy) => {
                cli.buy(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Sell) => {
                cli.sell(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::DockList) => {
                cli.dock_list(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Dock) => {
                cli.dock(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Undock) => {
                cli.undock(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Refuel) => {
                cli.refuel(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Name) => {
                cli.name(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Time) => {
                cli.time(cmd);
            }
            v if cli.check_cmd(v, CmdName::Save) => {
                cli.save(cmd, &entities);
            }
            v if cli.check_cmd(v, CmdName::Load) => {
                cli.load(cmd, &mut entities);
            }
            v if cli.check_cmd(v, CmdName::Quit) => {
                cli.quit(cmd);
                break;
            }
            _ => {
                cli.unknown(cmd);
            }
        }
    }
}
