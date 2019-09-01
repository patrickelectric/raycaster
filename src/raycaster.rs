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
    wall_size: f64,
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
            wall_size: 5.0,
            player: Player::default(),
        };
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = (self.map.len() as f64).sqrt() as usize;
        for u in 0..size {
            for i in 0..size {
                write!(f, "\t{}, ", self.map[u * size + i]);
            }
            write!(f, "\n");
        }
        write!(f, "----")
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
    pub fn draw(&mut self, texture: &mut piston_window::G2dTexture) {
        // Fov in radians
        let fov_rad: f64 = self.player.fov * std::f64::consts::PI / 180.0;
        // Only rectangles for now
        let cell_size = self.scale / self.size() as f64;
        // Scaled position
        let pos_scaled = (
            self.player.pos.0 / self.scale,
            self.player.pos.1 / self.scale,
        );

        let mut test = self.map.clone();

        // Total number of steps around fov angle
        let total_horizontal_steps = 10;
        for n in -total_horizontal_steps / 2..=total_horizontal_steps / 2 {
            let n_float = n as f64;
            // Total number of steps until hit or end of map
            for d in 1..10 {
                let d_float = d as f64;

                let angle = (fov_rad / 2.0) * (n as f64 / total_horizontal_steps as f64);
                let pos_new = (
                    (pos_scaled.0 + d_float * angle.cos()).round() as u64,
                    (pos_scaled.1 + d_float * angle.sin()).round() as u64,
                );
                // Check for invalid values
                if pos_new.0 < 0
                    || pos_new.1 < 0
                    || pos_new.0 >= self.size()
                    || pos_new.1 >= self.size()
                {
                    continue;
                }

                let map_index: usize = (pos_new.1 + self.size() * pos_new.0) as usize;
                if self.map[map_index] != 0 {
                    let projected_wall_height =
                        self.wall_size * self.player.projection_plane_distance / d_float;
                    println!("{:#?}, {}: {}", pos_new, angle, projected_wall_height);
                    break;
                }
                /*
                let map_pos: usize = (pos_new.1 + self.size() * pos_new.0) as usize;
                self.map[map_pos] = 3;
                println!("{:}", self);
                */

                /*
                println!("> {:#?}", (pos_new.1 + self.size() * pos_new.0) as usize);
                println!("{:#?}", pos_new);
                println!(
                    "{:#?}",
                    self.map[(pos_new.1 + self.size() * pos_new.0) as usize]
                );*/
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
