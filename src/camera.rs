use std::f32::consts;

use crate::{misc, vec::Vec2};
pub struct Camera {
    // [-1.0, 1.0], independent of terrain size
    pub x: f32,

    // [-1.0, 1.0], independent of terrain size
    pub y: f32,

    // How far our vision goes, [0.0, 1.0].
    pub viewing_distance: f32,

    // Radians, (-pi, pi]
    pub viewing_angle: f32,

    //
    pub viewing_dir: Vec2,

    // Radians: Angle centered on viewing angle.
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,

            viewing_distance: 0.25,

            // Straight "up"
            viewing_angle: consts::FRAC_PI_2,
            viewing_dir: Vec2::new_from_angle(consts::FRAC_PI_2),

            fov: consts::FRAC_PI_4,
        }
    }
}

impl Camera {
    pub fn rotate(&mut self, angle: f32) {
        self.viewing_angle += angle;
        self.viewing_dir = Vec2::new_from_angle(self.viewing_angle);
    }

    pub fn displace(&mut self, direction: misc::Direction, amount: f32) {
        let (x, y) = misc::displace(direction, amount);

        self.x += x;
        self.y += y;
    }

    // pub fn within_view(&self, x: f32, y: f32) -> bool {
    //     assert!((-1.0..=1.0).contains(&x));
    //     assert!((-1.0..=1.0).contains(&y));

    //     let vec_between = Vec2::new_from_points(self.x, self.y, x, y);
    //     let angle_between = self.viewing_dir.angle_other(&vec_between);

    //     let half_fov = self.fov / 2.0;

    //     let angle_ok = angle_between <= half_fov;
    //     let dist_ok = vec_between.magnitude() <= self.viewing_distance;

    //     angle_ok && dist_ok
    // }
}
