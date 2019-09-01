extern crate piston_window;

use std::vec::Vec;

pub struct Player {
    pos: (f64, f64),
    fov: f64,
    projection_plane_distance: f64,
}

impl Default for Player {
    fn default() -> Player {
        return Player {
            pos: (350.0, 350.0),
            fov: 135.0,
            projection_plane_distance: 10.0,
        };
    }
}

pub struct Environment {
    map: Vec<u64>,
    scale: f64,
    player: Player,
}

#[rustfmt::skip::macros(vec)]
impl Default for Environment {
    fn default() -> Environment {
        return Environment {
            map: vec![
                1, 1, 1, 1, 1, 1, 1,
                1, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 1,
                1, 0, 0, 0, 0, 0, 1,
                1, 1, 1, 1, 1, 1, 1,
            ],
            scale: 100.0,
            player: Player::default(),
        };
    }
}

/*
use std::convert::From;
impl From<(usize, usize)> for (u32, u32) {
    fn from(value: (usize, usize)) -> (u32, u32) {
        (value.0 as u32, value.1 as u32)
    }
}
*/

impl Environment {
    pub fn draw(&self, texture: &mut piston_window::G2dTexture) {
        // Fov in radians
        let fov_rad: f64 = self.player.fov * std::f64::consts::PI / 180.0;
        // Only rectangles for now
        let cell_size = self.scale / self.size() as f64;

        // Total number of steps around fov angle
        for n in -5..=5 {
            let n_float = n as f64;
            // Total number of steps until hit or end of map
            for d in 1..10 {
                let d_float = d as f64;

                //let new_pos = ((n_float*fov/10.0).cos()*d_float, (n_float*fov/10.0).sin()*d_float);
                //println!("{:#?}", new_pos);
            }
        }
    }

    pub fn set_map(&mut self, map: Vec<u64>) {
        self.map = map.clone();
    }

    pub fn is_position_inside(&self, pos: (u64, u64)) -> bool {
        let size = self.size();
        return pos > (0, 0) && pos < (size, size);
    }

    pub fn size(&self) -> u64 {
        return (self.map.len() as f64).sqrt() as u64;
    }
}
