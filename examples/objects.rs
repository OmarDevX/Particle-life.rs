use glm::{length, normalize, vec2, Vec2, Vec3};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub position: Vec2,   // center of the circle
    pub velocity: Vec2,   // velocity of the circle
    pub radius: f32,      // radius of the circle
    pub mass: f32,        // mass of the circle
    pub color: Vec3,      // color of the circle (RGB)
    pub friction: f32,    // friction coefficient
    pub speed_limit: f32, // maximum speed limit
    pub cell_type: i32,   // type of the circle
    pub predators: Vec<i32>,
    pub friends: Vec<i32>,
    pub foods: Vec<i32>,
    pub hunger: f32,
}

impl Circle {
    pub fn new(cell_type: i32, position: Vec2, radius: f32, color: Vec3, speed_limit: f32) -> Self {
        Circle {
            position,
            velocity: vec2(0.0, 0.0),
            radius,
            mass: 1.0, // default mass
            color,
            friction: 0.5, // default friction
            speed_limit,
            cell_type,
            predators: [].to_vec(),
            friends: [].to_vec(),
            foods: [].to_vec(),
            hunger: 100.0,
        }
    }

    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    pub fn update(&mut self, dt: f32) {
        // Apply friction
        self.velocity = self.velocity * (1.0 - self.friction * dt);
        // Limit velocity
        let current_speed = length(self.velocity);
        if current_speed > self.speed_limit {
            let direction = normalize(self.velocity);
            self.velocity = direction * self.speed_limit;
        }

        // Update position based on velocity
        self.position = self.position + self.velocity * dt;
    }
    pub fn eat(&mut self, object:&mut Circle){
        self.hunger+=10.0;
        object.cell_type=0;
    }
    pub fn decide(&mut self, objects: &mut Vec<Circle>) {
        let follow_radius = 0.5;
        let mut following = false;

        for object in objects.iter() {
            if *object != *self {
                // Calculate distance between self and object
                let dx = object.position.x - self.position.x;
                let dy = object.position.y - self.position.y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= follow_radius && distance != 0.0 {
                    if self.predators.contains(&object.cell_type) {
                        escape_particle(self, object, distance, 1.0, 1.0);
                    }
                    if self.friends.contains(&object.cell_type) {
                        follow_particle(self, object, distance, 1.0, 1.0);
                    }
                    if self.foods.contains(&object.cell_type) {
                        follow_particle(self, object, distance, 1.0, 1.0);
                    }
                    // Call follow_particle function
                    following = true;
                }
            }
        }

        if !following {
            // If not following any particle, move randomly
            self.move_randomly();
        }
    }

    fn move_randomly(&mut self) {
        let mut rng = rand::thread_rng();
        let random_angle: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let random_speed: f32 = rng.gen_range(0.0..self.speed_limit);

        self.velocity.x = random_speed * random_angle.cos();
        self.velocity.y = random_speed * random_angle.sin();
    }
}

fn follow_particle(
    particle: &mut Circle,
    target: &Circle,
    distance: f32,
    speed: f32,
    multiplier: f32,
) {
    let dx = target.position.x - particle.position.x;
    let dy = target.position.y - particle.position.y;

    let vx = (dx / distance) * speed * multiplier;
    let vy = (dy / distance) * speed * multiplier;

    particle.velocity.x += vx;
    particle.velocity.y += vy;
}
fn escape_particle(
    particle: &mut Circle,
    target: &Circle,
    distance: f32,
    speed: f32,
    multiplier: f32,
) {
    let dx = target.position.x - particle.position.x;
    let dy = target.position.y - particle.position.y;

    let vx = (dx / distance) * speed * multiplier;
    let vy = (dy / distance) * speed * multiplier;

    particle.velocity.x -= vx;
    particle.velocity.y -= vy;
}
