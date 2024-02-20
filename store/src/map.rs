const MAP1: [[u8; 16]; 9] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

pub struct Map {
    mini_map: [[u8; 16]; 9],
    world_map: [[u8; 16]; 9],
}

impl Map {
    pub fn new() -> Self {
        let mut new_map = Map { mini_map: MAP1, world_map: [[0; 16]; 9] };
        new_map.get_map();
        new_map
    }

    fn get_map(&mut self) {
        for (i, row) in self.mini_map.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if *value > 0 {
                    self.world_map[i][j] = *value;
                }
            }
        }
    }
}
