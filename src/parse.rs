use std::collections::HashMap;
use serde_json;
use data::Game;

#[derive(Serialize, Deserialize)]
pub enum Request {
    GetCache,
    UpdateGame(Game),
    RemoveGame,
}

pub fn get_request(req: &[u8]) -> Result<Request, serde_json::Error> {
    serde_json::from_slice(req)
}

pub fn make_cache(data: &Vec<Game>) -> String {
    serde_json::to_string(data)
        .expect("Error caching the game data.")
}