use std::f64::consts::PI;

pub struct Player {
    position: (f64, f64),
    fov: (f64, f64),
    direction: f64,
    view_distance: f64,
}

impl Player {
    pub fn new(position: (f64, f64), vertical_fov: f64, horizontal_fov: f64, view_distance: f64) -> Player {
        Player {
            position: position,
            fov: (vertical_fov, horizontal_fov),
            direction: 0.0,
            view_distance: view_distance,
        }
    }

    pub fn position(&self) -> (f64, f64) {
        self.position
    }

    pub fn move_camera_relative(&mut self, movement: (f64, f64)) {
        self.position.0 += self.direction.cos() * movement.0;
        self.position.1 += self.direction.sin() * movement.0;

        self.position.0 += (self.direction + -(PI / 2.0)).cos() * movement.1;
        self.position.1 += (self.direction + -(PI / 2.0)).sin() * movement.1;
    }

    pub fn vertical_fov(&self) -> f64 {
        self.fov.0
    }

    pub fn horizontal_fov(&self) -> f64 {
        self.fov.1
    }

    pub fn direction(&self) -> f64 {
        self.direction
    }

    pub fn turn(&mut self, angle: f64) {
        self.direction += angle;
    }

    pub fn view_distance(&self) -> f64 {
        self.view_distance
    }

}