use std::f32::consts;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn distance(&self, vec3: &Vec3) -> f32 {
        (
            (vec3.x - self.x).powi(2) +
                (vec3.y - self.y).powi(2) +
                (vec3.z - self.z).powi(2)
        ).sqrt()
    }

    pub fn angle(&self, vec3: &Vec3) -> Vec3 {
        Vec3 {
            x: ((-(vec3.x - self.x).atan2(vec3.y - self.y)) / consts::PI * 180.0),
            y: ((vec3.z - self.z) / self.distance(&vec3)).asin() * 180.0 / consts::PI,
            z: 0.0
        }
    }
}