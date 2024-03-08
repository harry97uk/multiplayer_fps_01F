use std::{ collections::HashMap };

use serde::{ Serialize, Deserialize };

use crate::{ map::Map, player::{ Player, PlayerDirection } };

type PlayerId = u64;

pub struct GameState {
    pub map: Map,
    pub players: HashMap<PlayerId, Player>,
    history: Vec<GameEvent>,
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
    PlayerMoved {
        player_id: PlayerId,
        new_position: (f32, f32),
        new_direction: (PlayerDirection, f32), //the f32 is the rotation amount in radians
    },
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            map: Map::new(),
            players: HashMap::default(),
            history: vec![],
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
            PlayerMoved { player_id, new_position, .. } => {
                //make sure it is a valid position to move to
                if
                    !self.players.contains_key(player_id) ||
                    self.map.mini_map[new_position.1 as usize][new_position.0 as usize] == 1
                {
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
            PlayerMoved { player_id, new_position, new_direction } => {
                self.players.get_mut(player_id).expect("no player with id").x = new_position.0;
                self.players.get_mut(player_id).expect("no player with id").z = new_position.1;
                self.players.get_mut(player_id).expect("no player with id").direction =
                    new_direction.0;
            }
        }

        self.history.push(event.clone());
    }

    pub fn get_player_position(&self, player_id: PlayerId) -> (f32, f32) {
        let player = self.players.get(&player_id).expect("no player with id");
        return (player.x, player.z);
    }
}
