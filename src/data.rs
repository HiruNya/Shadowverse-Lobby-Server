/// Stores all the structs that are used

use std::collections::HashMap;

pub struct GameData {
    games: HashMap<String, Game>,
    cache: String,
}

impl GameData {
    pub fn new() -> GameData {
        let mut game_data = GameData {
            games: HashMap::new(),
            cache: String::new(),
        };
        game_data.update_cache();
        game_data
    }
    // ToDo
    pub fn update_cache(&mut self) {
        self.cache = String::new()
    }
}

struct Game;