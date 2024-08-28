pub mod windows {
    use ahash::{HashMap, HashMapExt};
    use egui::{Modifiers, Ui};
    use rand::Rng;
    use crate::{objects, Circle};
    use glm::*;

    #[derive(Clone)]
    pub struct SandboxWindow {
        pub objects: Vec<Circle>,
        pub default_object: Circle,
        spawn_objects_count:i32,
    }

    impl SandboxWindow {
        pub fn new() -> Self {
            Self {
                objects: [].to_vec(),
                default_object: Circle {
                    position: vec2(0.5, 0.5),
                    velocity: vec2(0.0, 0.0),
                    radius: 0.01,
                    mass: 1.0,
                    color: vec3(225.0, 0.0, 0.0),
                    friction: 0.01,
                    speed_limit: 0.0001,
                    cell_type: 0,
                    predators: [].to_vec(),
                    friends: [].to_vec(),
                    foods: [].to_vec(),
                    hunger: 100.0,
                },
                spawn_objects_count: 10,
            }
        }

        pub fn ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
            let _ = ctx;
            self.scene_settings(ui);
        }

        pub fn scene_settings(&mut self, ui: &mut Ui) {
            ui.label("Circle Settings");

            // Position
            ui.horizontal(|ui| {
                ui.label("Cell Type:");
                ui.add(egui::DragValue::new(&mut self.default_object.cell_type).speed(1));
            });

            // Position
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.add(egui::DragValue::new(&mut self.default_object.position.x).speed(0.01).prefix("x:"));
                ui.add(egui::DragValue::new(&mut self.default_object.position.y).speed(0.01).prefix("y:"));
            });

            // Velocity
            ui.horizontal(|ui| {
                ui.label("Velocity:");
                ui.add(egui::DragValue::new(&mut self.default_object.velocity.x).speed(0.01).prefix("x:"));
                ui.add(egui::DragValue::new(&mut self.default_object.velocity.y).speed(0.01).prefix("y:"));
            });

            // Radius
            ui.horizontal(|ui| {
                ui.label("Radius:");
                ui.add(egui::DragValue::new(&mut self.default_object.radius).speed(0.01));
            });

            // Mass
            ui.horizontal(|ui| {
                ui.label("Mass:");
                ui.add(egui::DragValue::new(&mut self.default_object.mass).speed(0.01));
            });

            // Color
            ui.horizontal(|ui| {
                ui.label("Color:");
                ui.add(egui::DragValue::new(&mut self.default_object.color.x).speed(1.0).prefix("R:"));
                ui.add(egui::DragValue::new(&mut self.default_object.color.y).speed(1.0).prefix("G:"));
                ui.add(egui::DragValue::new(&mut self.default_object.color.z).speed(1.0).prefix("B:"));
            });

            // Friction
            ui.horizontal(|ui| {
                ui.label("Friction:");
                ui.add(egui::DragValue::new(&mut self.default_object.friction).speed(0.01));
            });

            // Speed Limit
            ui.horizontal(|ui| {
                ui.label("Speed Limit:");
                ui.add(egui::DragValue::new(&mut self.default_object.speed_limit).speed(0.01));
            });

            // Add Circle Button
            if ui.button("Generate World").clicked() {
                for _ in 0..self.spawn_objects_count {
                    let mut new_object = self.default_object.clone();
                
                    new_object.cell_type = rand::thread_rng().gen_range(1..=5); // Generates a number between 1 and 5 (inclusive)

                    match new_object.cell_type {
                        1 => {
                            // red - Aggressive type, hunts for food, but can have alliances
                            new_object.color = vec3(225.0, 0.0, 0.0);
                            new_object.friends = vec![2, 4];  // Allies with blue and soil cells
                            new_object.foods = vec![5];       // Hunts food cells
                            new_object.predators = vec![3];   // Preyed upon by white cells
                        }
                        2 => {
                            // blue - Defensive type, avoids conflict, gathers resources
                            new_object.color = vec3(0.0, 225.0, 0.0);
                            new_object.friends = vec![4, 1];  // Allies with soil and red cells
                            new_object.foods = vec![5];       // Gathers food cells
                            new_object.predators = vec![3,2];   // Preyed upon by white cells
                        }
                        3 => {
                            // white - Dominant type, aggressive, preys on others
                            new_object.color = vec3(0.0, 0.0, 255.0);
                            new_object.friends = vec![3, 5];  // Allies with its own kind and food cells
                            new_object.foods = vec![1, 2];    // Preys on red and blue cells
                            new_object.predators = vec![4];   // Soil cells can neutralize it
                        }
                        4 => {
                            // soil - Neutral type, supports others, but can be defensive
                            new_object.color = vec3(0.0, 225.0, 255.0);
                            new_object.friends = vec![1, 2, 4];  // Allies with red, blue, and its own kind
                            new_object.foods = vec![3];          // Can neutralize white cells
                            new_object.predators = vec![5];      // Food cells can be invasive
                        }
                        5 => {
                            // food - Essential resource, tries to survive, invasive tendencies
                            new_object.color = vec3(255.0, 225.0, 0.0);
                            new_object.friends = vec![3, 5];  // Allies with white cells and its own kind
                            new_object.foods = vec![2];       // Competes with blue cells
                            new_object.predators = vec![1, 4];  // Preyed upon by red and soil cells
                        }
                        _ => {
                            // default - Unknown type, neutral behavior
                            new_object.color = vec3(255.0, 255.0, 255.0);
                        }
                    }
                    
                    new_object.position = vec2(
                        rand::random::<f32>() * 0.8 - 0.1, // random x between 0.1 and 0.9
                        rand::random::<f32>() * 0.8 - 0.1, // random y between 0.1 and 0.9
                    );
                    self.objects.push(new_object);
                }
            }
            if ui.button("Add Circle").clicked(){
                self.objects.push(self.default_object.clone());
            }
        }

        pub fn update_objects(&mut self, dt: f32, window_width: f32, window_height: f32) {
            // Create a separate vector to hold updated objects
            let mut updated_objects = Vec::new();

            for object in &mut self.objects {
                object.update(dt);
                updated_objects.push(object.clone());
            }

            // Now handle border collisions for each updated object
            for object in &mut updated_objects {
                // self.handle_border_collision(object, window_width, window_height);
                object.decide(&mut self.objects);
            }

            // Replace the objects with the updated objects
            self.objects = updated_objects;
            // Handle collisions between circles
            handle_collisions(&mut self.objects);
        }
    
    }

    pub struct MainWindow<'a> {
        pub show_sandbox_window: bool,
        pub sandbox_window: &'a mut SandboxWindow,
    }

    impl<'a> MainWindow<'a> {
        pub fn new(sandbox_window: &'a mut SandboxWindow) -> Self {
            Self {
                show_sandbox_window: false,
                sandbox_window,
            }
        }

        pub fn ui(&mut self, ctx: &egui::Context) {
            self.desktop_ui(ctx);
        }

        pub fn desktop_ui(&mut self, ctx: &egui::Context) {
            egui::SidePanel::left("egui_demo_panel")
                .resizable(true)
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("✒ Vetracer2D");
                    });
                    ui.separator();
                    use egui::special_emojis::GITHUB;
                    if self.show_sandbox_window {
                        egui::Window::new("Sandbox Window")
                            .resizable(true)
                            .default_width(400.0)
                            .show(ctx, |ui| {
                                self.sandbox_window.ui(ctx, ui);
                            });
                    }
                    ui.hyperlink_to(
                        format!("{GITHUB} Resource Code"),
                        "https://github.com/OmarDevX",
                    );
                    ui.separator();
                    self.demo_list_ui(ui);
                });

            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    file_menu_button(ui);
                });
            });
        }

        pub fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    ui.label("Menu");
                    if ui.button("Default Window").clicked() {
                        self.show_sandbox_window = !self.show_sandbox_window;
                    }

                    if ui.button("Organize windows").clicked() {
                        ui.ctx().memory_mut(|mem| mem.reset_areas());
                    }
                });
            });
        }
    }

    pub fn file_menu_button(ui: &mut Ui) {
        let organize_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
        let reset_shortcut =
            egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

        if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
        }

        if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
        }

        ui.menu_button("File", |ui| {
            ui.set_min_width(220.0);
            ui.style_mut().wrap = Some(false);

            #[cfg(not(target_arch = "wasm32"))]
            {
                egui::gui_zoom::zoom_menu_buttons(ui);
                ui.weak(format!(
                    "Current zoom: {:.0}%",
                    100.0 * ui.ctx().zoom_factor()
                ))
                .on_hover_text("The UI zoom level, on top of the operating system's default value");
                ui.separator();
            }

            if ui
                .add(
                    egui::Button::new("Organize Windows")
                        .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
                )
                .clicked()
            {
                ui.ctx().memory_mut(|mem| mem.reset_areas());
                ui.close_menu();
            }

            if ui
                .add(
                    egui::Button::new("Reset egui memory")
                        .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
                )
                .on_hover_text("Forget scroll, positions, sizes etc")
                .clicked()
            {
                ui.ctx().memory_mut(|mem| *mem = Default::default());
                ui.close_menu();
            }
        });
    }

fn handle_collisions(objects: &mut Vec<Circle>) {
    let len = objects.len();
    for i in 0..len {
        let (left, right) = objects.split_at_mut(i + 1);
        let obj1 = &mut left[i];
        for obj2 in right.iter_mut() {
            let delta = obj1.position - obj2.position;
            let distance = glm::length(delta);
            let min_distance = obj1.radius + obj2.radius;

            if distance < min_distance {
                let normal = glm::normalize(delta);
                let penetration_depth = min_distance - distance;

                // Resolve the collision by moving the circles apart
                let total_mass = obj1.mass + obj2.mass;
                obj1.position = obj1.position + normal * penetration_depth * (obj2.mass / total_mass);
                obj2.position = obj2.position - normal * penetration_depth * (obj1.mass / total_mass);

                // Calculate the relative velocity
                let relative_velocity = obj1.velocity - obj2.velocity;
                let velocity_along_normal = glm::dot(relative_velocity, normal);

                if velocity_along_normal > 0.0 {
                    continue;
                }

                // Calculate the impulse scalar
                let restitution = 10.8; // coefficient of restitution
                let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
                let impulse_scalar = impulse_scalar / (1.0 / obj1.mass + 1.0 / obj2.mass);

                // Apply the impulse to the velocities

                let impulse = glm::vec2(impulse_scalar * normal.x, impulse_scalar * normal.y);
                obj1.velocity = obj1.velocity + impulse / obj1.mass;
                obj2.velocity = obj2.velocity - impulse / obj2.mass;
                if(obj1.foods.contains(&obj2.cell_type)){
                    obj1.eat(obj2);
                }
                if(obj2.foods.contains(&obj1.cell_type)){
                    obj2.eat(obj1);
                }
            }
        }
    }
}



}
