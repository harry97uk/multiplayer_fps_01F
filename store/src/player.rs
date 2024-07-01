use glam::f32::Vec3;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerDirection {
    North,
    East,
    South,
    West,
}

impl PlayerDirection {
    pub fn value(&self) -> Vec3 {
        match *self {
            PlayerDirection::East => Vec3::new(1.0, 0.0, 0.0),
            PlayerDirection::West => Vec3::new(-1.0, 0.0, 0.0),
            PlayerDirection::North => Vec3::new(0.0, 0.0, -1.0),
            PlayerDirection::South => Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

pub enum PlayerStartingPositions {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
}

pub struct Player {
    pub id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub direction: PlayerDirection,
}

impl Player {
    pub fn new(
        id: u64,
        name: String,
        starting_position: PlayerStartingPositions,
        map_size: (usize, usize)
    ) -> Self {
        let (x, z, direction) = match starting_position {
            PlayerStartingPositions::TopLeft => (1.0, 1.0, PlayerDirection::East),
            PlayerStartingPositions::BottomLeft =>
                (1.0, (map_size.1 as f32) - 2.0, PlayerDirection::East),
            PlayerStartingPositions::TopRight =>
                ((map_size.0 as f32) - 2.0, 1.0, PlayerDirection::West),
            PlayerStartingPositions::BottomRight =>
                ((map_size.0 as f32) - 2.0, (map_size.1 as f32) - 2.0, PlayerDirection::West),
        };
        Player {
            id,
            name,
            x,
            y: 0.2,
            z,
            direction,
        }
    }
}
