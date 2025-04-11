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
                (rng.random_range(0.1..1.0) as f32 * 100 as f32).round() / 100 as f32
            })
        });

        for (rows_key, rows) in self.map_box.iter_mut().enumerate() 
        {
            for (columns_key, column) in rows.iter_mut().enumerate()
            {
                dbg!(columns_key, column);
            }
        }

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
