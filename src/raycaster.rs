extern crate piston_window;

use crate::piston_window::Transformed;
use std::vec::Vec;

pub struct Player {
    pub pos: (f64, f64),
    pub angle: f64,
    fov: f64,
    projection_plane_distance: f64,
}

impl Default for Player {
    fn default() -> Player {
        return Player {
            pos: (350.0, 350.0),
            angle: 0.0,
            fov: 135.0,
            projection_plane_distance: 10.0,
        };
    }
}

pub struct Environment {
    map: Vec<u64>,
    scale: f64,
    wall_size: f64,
    pub player: Player,
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
            wall_size: 20.0,
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
    pub fn draw(
        &mut self,
        context: &mut piston_window::Context,
        graphics: &mut piston_window::G2d,
    ) {
        let window_size = context.viewport.unwrap().window_size;
        let image_size = [320.0, 400.0];

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
        let total_horizontal_steps = 100;
        for n in -total_horizontal_steps / 2..=total_horizontal_steps / 2 {
            let n_float = n as f64;
            // Total number of steps until hit or end of map
            for d in 1..100 {
                let angle = (fov_rad / 2.0) * (n as f64 / total_horizontal_steps as f64);
                let d_float = d as f64;
                let pos_new_f = (
                    (pos_scaled.0 + d_float * angle.cos()) * self.player.angle.cos(),
                    (pos_scaled.1 + d_float * angle.sin()) * self.player.angle.sin(),
                );
                let real_distance = pos_new_f.0.hypot(pos_new_f.1);
                let pos_new = (pos_new_f.0.round() as u64, pos_new_f.1.round() as u64);

                // Check for invalid values
                if pos_new.0 < 0
                    || pos_new.1 < 0
                    || pos_new.0 >= self.size()
                    || pos_new.1 >= self.size()
                {
                    continue;
                }

                let x_pos = image_size[0] / 2.0
                    + image_size[0] * (n as f64) / (total_horizontal_steps as f64);

                let map_index: usize = (pos_new.1 + self.size() * pos_new.0) as usize;
                if self.map[map_index] != 0 {
                    let projected_wall_height =
                        self.wall_size * self.player.projection_plane_distance / real_distance;
                    //println!("> {:#?}, {}: {}", pos_new_f, angle, projected_wall_height);
                    piston_window::rectangle(
                        [1.0, 0.0, 0.0, 1.0],
                        [
                            x_pos - projected_wall_height / 2.0,
                            image_size[1] / 2.0 - projected_wall_height / 2.0,
                            projected_wall_height,
                            projected_wall_height,
                        ],
                        context.transform.scale(
                            window_size[0] / image_size[0] as f64,
                            window_size[1] / image_size[1] as f64,
                        ),
                        graphics,
                    );
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

    pub fn draw_mini_map(
        &mut self,
        context: &mut piston_window::Context,
        graphics: &mut piston_window::G2d,
    ) {
        let window_size = context.viewport.unwrap().window_size;
        let image_size = [320.0, 400.0];
        let mini_map_rect = [image_size[0]*0.8, 0.0, image_size[0]*0.2, image_size[1]*0.2];
        piston_window::rectangle(
            [1.0, 1.0, 1.0, 1.0],
            mini_map_rect,
            context.transform.scale(
                window_size[0] / image_size[0] as f64,
                window_size[1] / image_size[1] as f64,
            ),
            graphics,
        );

        for cell in 0..self.map.len() {
            if self.map[cell] == 0 {
                continue;
            }

            let size = self.size();
            let x = (cell as u64 % size) as f64;
            let y = (cell as f64 / size as f64).floor() as f64;
            let mini_cell_size = (
                (mini_map_rect[2]/size as f64) as f64,
                (mini_map_rect[3]/size as f64) as f64,
            );

            piston_window::rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [
                    mini_map_rect[0] + mini_cell_size.0*x, mini_map_rect[1] + mini_cell_size.1*y, mini_cell_size.0, mini_cell_size.1
                ],
                context.transform.scale(
                    window_size[0] / image_size[0] as f64,
                    window_size[1] / image_size[1] as f64,
                ),
                graphics,
            );
            let pos_scale = (
                self.player.pos.0*image_size[0]/(self.size() as f64* self.scale),
                self.player.pos.1*image_size[1]/(self.size() as f64* self.scale)
            );
            piston_window::rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [
                    mini_map_rect[0] + pos_scale.0*0.2 - 2.0, pos_scale.1*0.2 - 2.0, 4.0, 4.0
                ],
                context.transform.scale(
                    window_size[0] / image_size[0] as f64,
                    window_size[1] / image_size[1] as f64,
                ),
                graphics,
            );
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

    pub fn move_player(&mut self, delta: (f64, f64)) {
        self.player.pos = (self.player.pos.0 + delta.0, self.player.pos.1 + delta.1);
    }

    pub fn rotate_player(&mut self, angle: f64) {
        self.player.angle += angle;
    }
}
