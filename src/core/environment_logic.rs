use rand::{self, rngs::StdRng, Rng, SeedableRng};

const MAP_SIZE: usize = 30;

pub struct EnvironmentLogic {
    seed: u64,
    // game_mod: i8,
    map_box: [[f32; MAP_SIZE]; MAP_SIZE],
    tile_size: f32,
}

impl EnvironmentLogic {
    pub fn create(seed: u64, _game_mod: i8, tile_size: f32) -> Self {
        let mut env = Self {
            seed,
            // game_mod,
            map_box: [[0.0; MAP_SIZE]; MAP_SIZE],
            tile_size,
        };
        env.generate();
        env
    }

    fn generate(&mut self) {
        // generate base ground
        // generate random map
        if self.seed == 0 {
            self.seed = rand::rng().random()
        }
        let mut rng = StdRng::seed_from_u64(self.seed as u64);

        self.map_box = std::array::from_fn(|_| {
            std::array::from_fn(|_| {
                (rng.random_range(0.5..1.0) as f32 * 100 as f32).round() / 100 as f32
            })
        });

        let mut old_map = self.map_box.clone();

        for (rows_key, rows) in self.map_box.iter_mut().enumerate() {
            for (columns_key, _column) in rows.iter_mut().enumerate() {
                if rows_key % 2 == 0 || columns_key % 2 == 0 {
                    if rows_key >= 1 && rows_key + 1 <= old_map.len()
                        && columns_key >= 1 && columns_key + 1 <= old_map[0].len()
                    {
                        let slice_c = &old_map[rows_key][columns_key-1..columns_key+1];
                        let slice_r: Vec<f32> = old_map[rows_key-1..rows_key+1]
                            .iter()
                            .map(|row| row[columns_key])
                            .collect();

                        let avg_c: f32 = slice_c.iter().copied().sum::<f32>() / slice_c.len() as f32;
                        let avg_r: f32 = slice_r.iter().copied().sum::<f32>() / slice_r.len() as f32;

                        old_map[rows_key][columns_key] = (avg_c + avg_r) /2f32;
                    }
                }
            }
        }

        self.map_box = old_map;
    }

    pub fn get_env(&self) -> [[f32; MAP_SIZE]; MAP_SIZE] {
        self.map_box
    }

    pub fn get_map_size(&self) -> usize {
        self.map_box.len()
    }

    pub fn get_tile_size(&self) -> f32 {
        self.tile_size
    }
}
