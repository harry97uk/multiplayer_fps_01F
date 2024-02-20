use std::{ collections::HashMap };

use serde::{ Serialize, Deserialize };

use crate::{ map::Map, player::Player };

type PlayerId = u64;

pub struct GameState {
    map: Map,
    pub players: HashMap<PlayerId, Player>,
    history: Vec<GameEvent>,
    active: bool,
}

/// An event that progresses the GameGameState forward
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum GameEvent {
    PlayerJoined {
        player_id: PlayerId,
        name: String,
    },
    PlayerDisconnected {
        player_id: PlayerId,
    },
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            map: Map::new(),
            players: HashMap::default(),
            history: vec![],
            active: true,
        }
    }

    /// Determines whether an event is valid considering the current GameState
    pub fn validate(&self, event: &GameEvent) -> bool {
        use GameEvent::*;
        match event {
            PlayerJoined { player_id, name: _ } => {
                if self.players.contains_key(player_id) || self.players.len() >= 4 {
                    return false;
                }
            }
            PlayerDisconnected { player_id, .. } => {
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }
        }
        true
    }

    /// Aggregates an event into the GameState.
    pub fn consume(&mut self, event: &GameEvent) {
        use GameEvent::*;
        match event {
            PlayerJoined { player_id, name } => {
                self.players.insert(*player_id, Player::new(*player_id, name.to_string()));
            }
            PlayerDisconnected { player_id, .. } => {
                self.players.remove(player_id);
            }
        }

        self.history.push(event.clone());
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.active {}

        Ok(())
    }
}
