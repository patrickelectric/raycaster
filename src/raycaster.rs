extern crate piston_window;

use std::vec::Vec;
use std::convert::From;

pub struct Environment {
    map: Vec<Vec<bool>>,
    scale: u32,
}

/*
impl From<(usize, usize)> for (u32, u32) {
    fn from(value: (usize, usize)) -> (u32, u32) {
        (value.0 as u32, value.1 as u32)
    }
}
*/

impl Environment {
    pub fn draw(&self, texture: piston_window::G2dTexture, pos: (u32, u32)) {
        let fov: f64 = 135.0*std::f64::consts::PI/180.0;
        // Only rectangles for now
        let cell_size = self.scale/self.map.len() as u32;

        // Horizontal scan
        for n in -5..=5 {
            // Distance scan
            for d in 1..10 {
                let new_pos = ((n as f64*fov/10.0).cos()*d as f64, (n as f64*fov/10.0).sin()*d as f64);
                //texture.surface.0;
            }
        }
    }

    pub fn is_position_inside(&self, pos: (u32, u32)) -> bool {
        let size = self.size();
        return pos < (size.0 as u32*self.scale, size.1 as u32*self.scale) && pos > (0, 0);
    }

    pub fn size(&self) -> (usize, usize) {
        return (self.map.len(), self.map[0].len());
    }
}
