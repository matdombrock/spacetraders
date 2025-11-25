#![allow(unused_variables)]
#![allow(dead_code)]

mod item {
    #[derive(Debug, PartialEq)]
    pub enum ItemName {
        Copper,
        Iron,
    }
    #[derive(Debug)]
    pub struct Item {
        pub name: ItemName,
        pub description: String,
        pub pc_vol: i32,
        pub stack: i32,
    }
    impl Item {
        pub fn volume(&self) -> i32 {
            self.pc_vol * self.stack
        }
    }
}

mod inventory {
    use crate::item::{Item, ItemName};

    #[derive(Debug)]
    pub struct Inventory {
        items: Vec<Item>,
        vol: i32,
    }
    impl Inventory {
        pub fn new() -> Self {
            Inventory {
                items: Vec::new(),
                vol: 0,
            }
        }
        pub fn find_mut(&mut self, name: &ItemName) -> Option<&mut Item> {
            for item in &mut self.items {
                if &item.name == name {
                    return Some(item);
                }
            }
            None
        }
        pub fn find(&self, name: &ItemName) -> Option<&Item> {
            for item in &self.items {
                if &item.name == name {
                    return Some(item);
                }
            }
            None
        }
        pub fn add(&mut self, item: Item) {
            if let Some(target) = self.find_mut(&item.name) {
                target.stack += item.stack;
            } else {
                self.items.push(item);
            }
            self.vol = self.calc_volume();
        }
        pub fn remove(&mut self, name: &ItemName, quantity: i32) {
            if let Some(target) = self.find_mut(name) {
                if target.stack >= quantity {
                    target.stack -= quantity;
                } else {
                    // Remove target completely if not enough quantity
                    self.items.retain(|item| &item.name != name);
                }
            }
        }
        fn calc_volume(&self) -> i32 {
            self.items.iter().map(|item| item.volume()).sum()
        }
        pub fn print(&self) {
            println!("Inventory:");
            for item in &self.items {
                println!("{item:?}");
            }
            println!("Total Volume: {}", self.vol);
        }
    }
}

mod items {
    use crate::item::{Item, ItemName};

    pub fn copper() -> Item {
        Item {
            name: ItemName::Copper,
            description: "A piece of copper".to_string(),
            pc_vol: 1,
            stack: 1,
        }
    }
    pub fn iron() -> Item {
        Item {
            name: ItemName::Iron,
            description: "A piece of iron".to_string(),
            pc_vol: 2,
            stack: 1,
        }
    }
}

mod pos {
    use rand::Rng;
    #[derive(Debug)]
    pub struct Position {
        x: i32,
        y: i32,
        z: i32,
    }
    impl Position {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Position { x, y, z }
        }
        pub fn random(max: i32) -> Self {
            let mut rng = rand::rng();
            Position {
                x: rng.random_range(0..max),
                y: rng.random_range(0..max),
                z: rng.random_range(0..max),
            }
        }
        pub fn distance(&self, other: &Position) -> f64 {
            let dx = (self.x - other.x) as f64;
            let dy = (self.y - other.y) as f64;
            let dz = (self.z - other.z) as f64;
            (dx * dx + dy * dy + dz * dz).sqrt()
        }
    }
}

mod system {
    use crate::inventory::Inventory;
    use crate::pos;
    use crate::ship::Ship;
    #[derive(Debug)]
    pub struct System {
        name: String,
        position: pos::Position,
        inventory: Inventory,
    }
    impl System {
        pub fn new() -> Self {
            System {
                name: System::gen_name(),
                position: System::gen_position(),
                inventory: System::gen_inventory(),
            }
        }
        fn gen_name() -> String {
            "Generated System".to_string()
        }
        fn gen_position() -> pos::Position {
            pos::Position::random(1000)
        }
        fn gen_inventory() -> Inventory {
            Inventory::new()
        }
        pub fn print(&self, player_ship: &Ship) {
            println!("Name: {}", self.name);
            println!("Position: {:?}", self.position);
            println!("Inventory: {:?}", self.inventory);
            let distance = self.position.distance(player_ship.position());
            println!("Distance from player: {:.2}", distance);
        }
    }
    #[derive(Debug)]
    pub struct List(Vec<System>);
    impl List {
        pub fn new() -> Self {
            List(Vec::new())
        }
        pub fn add(&mut self, system: System) {
            self.0.push(system);
        }
        pub fn find_by_name(&self, name: &str) -> Option<&System> {
            for system in &self.0 {
                if system.name == name {
                    return Some(system);
                }
            }
            None
        }
        pub fn find_by_distance(
            &self,
            position: &pos::Position,
            max_distance: f64,
        ) -> Vec<&System> {
            let mut results = Vec::new();
            for system in &self.0 {
                let distance = system.position.distance(position);
                if distance <= max_distance {
                    results.push(system);
                }
            }
            results
        }
        pub fn print(&self, player_ship: &Ship) {
            for system in &self.0 {
                println!("-------------------------");
                system.print(player_ship);
            }
        }
    }
}

mod entity {
    #[derive(Debug)]
    pub struct Entity {
        name: String,
        credits: i32,
    }
    impl Entity {
        pub fn new(name: String, credits: i32) -> Self {
            Entity { name, credits }
        }
    }
}

mod ship {
    use crate::entity::Entity;
    use crate::inventory::Inventory;
    use crate::pos::Position;
    #[derive(Debug)]
    pub enum ShipClass {
        Fighter,
        Freighter,
        Explorer,
    }
    #[derive(Debug)]
    pub struct Ship {
        name: String,
        class: ShipClass,
        pilot: Entity,
        hull: i32,
        shields: i32,
        hold_volume: i32,
        jump_range: i32,
        position: Position,
        pub inventory: Inventory,
    }
    impl Ship {
        pub fn new(name: String, class: ShipClass, pilot: Entity) -> Self {
            Ship {
                name,
                class,
                pilot,
                hull: 100,
                shields: 100,
                hold_volume: 1000,
                jump_range: 10000,
                position: Position::new(0, 0, 0),
                inventory: Inventory::new(),
            }
        }
        pub fn position(&self) -> &Position {
            &self.position
        }
        pub fn jump_range(&self) -> i32 {
            self.jump_range
        }
    }
}

mod sc_inventory {
    use crate::ship::Ship;
    pub fn sc(ship: &Ship) {
        ship.inventory.print();
    }
}

mod sc_system_list {
    use crate::ship::Ship;
    use crate::system::List;
    pub fn sc(systems: &List, player_ship: &Ship) {
        println!("Star Systems:");
        systems.print(player_ship);
    }
}

mod sc_jump {
    use crate::ship::Ship;
    use crate::system::List;
    pub fn sc(ship: &Ship, systems: &List) {
        let jumpable = systems.find_by_distance(ship.position(), ship.jump_range() as f64);
        println!("Jumpable Systems:");
        for sys in jumpable {
            println!("-------------------------");
            sys.print(ship);
        }
    }
}

use std::io::{self, Write};

fn main() {
    let copper = items::copper();
    let iron = items::iron();

    let mut player = ship::Ship::new(
        "SS Voyager".to_string(),
        ship::ShipClass::Explorer,
        entity::Entity::new("Captain Reynolds".to_string(), 10000),
    );
    player.inventory.add(copper);
    player.inventory.add(iron);

    let sys_count = 5;
    let mut systems = system::List::new();
    for _ in 0..sys_count {
        let sys = system::System::new();
        systems.add(sys);
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        let command = inp.trim();
        match command {
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            "inventory" | "inv" | "i" => {
                sc_inventory::sc(&player);
            }
            "systems" | "sys" | "s" => {
                sc_system_list::sc(&systems, &player);
            }
            "jump" | "j" => {
                sc_jump::sc(&player, &systems);
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
