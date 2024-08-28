use glm::Vec2;

pub struct Camera{
    pub offset: Vec2,
    pub zoom: f32,
    pub velocity: Vec2,
    pub friction: f32, // new field to store friction coefficient

}

impl Camera {
    pub fn new(offset: Vec2, zoom: f32, velocity: Vec2, friction: f32) -> Self {
        Self { offset, zoom, velocity, friction }
    }
    pub fn update(&mut self, delta_time: f32) {
        // apply friction to velocity
        self.velocity =self.velocity * (1.0 - self.friction * 0.0016);
        self.offset = self.offset + self.velocity * 0.0016;
    }
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
    pub fn get_offset(&self) -> Vec2 {
        self.offset
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
}
fn main(){
    
}