use glam::f32::Vec3;

#[derive(Clone)]
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

pub struct Player {
    id: u64,
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub direction: PlayerDirection,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Player { id, name, x: 0, y: 0, direction: PlayerDirection::East }
    }

    fn movement(&self) {
        let mut dx = 0;
        let mut dy = 0;
    }

    fn update(&self) {
        self.movement();
    }

    fn render(&mut self) {}
}
