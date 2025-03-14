use std::env;

use rand::{self, rngs::StdRng, Rng, SeedableRng};

const mapSize : usize = 30;

pub struct EnvirementLogic {
    seed : u64,
    gameMod : i8,
    mapBox: [[f32 ; mapSize]; mapSize],
    tileSize:f32
}

impl EnvirementLogic{
    pub fn create(seed:u64 , gameMod:i8 ,tileSize:f32) -> Self {
        let mut env = Self{
        seed,
        gameMod,
        mapBox: [[0.0; mapSize]; mapSize],   
        tileSize
        };
        env.generate();
        env
    }

    fn generate(&mut self){
                
        // generate base ground
            // generate random map
            if self.seed == 0 {self.seed = rand::thread_rng().gen()}
            let mut rng = StdRng::seed_from_u64(self.seed as u64);
            
            self.mapBox = std::array::from_fn(|_| 
            std::array::from_fn(|_|  
                ((rng.gen_range(-1.0..1.0) as f32*
                 mapSize as f32).round() / mapSize as f32)
                )
            );
        //generate cordinate that have radio active noise 

        //genetare cordinate that have good ground for 

    }

    pub fn get_env(&self) -> [[f32; mapSize]; mapSize]{
        self.mapBox
    }

    pub fn get_mapSize(&self) -> usize {
        self.mapBox.len()
    }

    pub fn get_tileSize(&self) -> f32{
        self.tileSize
    }

}