use bevy::prelude::*;


#[derive(Resource)]
pub struct LevelManager {
    pub levels: Vec<Level>,
}

impl Default for LevelManager {
    fn default() -> Self {
        let amount = 10;
        let levels: Vec<Level> = vec![Level::new(10, 10); amount];  

        LevelManager {
            levels
        }
    }
}


#[derive(Clone)]
pub struct Level {
    pub level: Vec<Vec<char>>,
}


impl Level {
    pub fn print_level(&self) {
        println!("well now what do we have here {:?}", self.level);
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut level = vec![vec!['0'; width]; height];

        for y in 0..height {
            for x in 0..width {
                level[y][x] = '0';
            }
        }

        Level {
            level,
        }
    }
}