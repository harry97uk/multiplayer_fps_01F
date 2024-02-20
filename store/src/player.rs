pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Player {
    id: u64,
    pub name: String,
    x: u32,
    y: u32,
    direction: Direction,
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Player { id, name, x: 0, y: 0, direction: Direction::East }
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
