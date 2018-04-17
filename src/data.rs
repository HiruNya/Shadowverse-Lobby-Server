/// Stores all the structs that are used
use std::collections::HashMap;

use parse;

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub games: HashMap<String, Game>,
    pub cache: String,
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
    pub fn update_cache(&mut self) {
        let mut new_cache: Vec<Game> = Vec::new();
        let mut map = self.games.clone();
        for (_, v) in  map.drain() {
            new_cache.push(v)
        }
        self.cache = parse::make_cache(&new_cache);

    }
    pub fn remove_game(&mut self, key: &String) {
        self.games.remove(key);
        self.update_cache();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub name: String,
    pub author: String,
    pub join_code: String,
}
impl Game {
    pub fn new(name: String, author: String, join_code: String) -> Game {
        Game {
            name,
            author,
            join_code,
        }
    }
}