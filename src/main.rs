use macroquad::prelude::*;
use std::{process::exit, time::Instant};

#[derive(Clone, Copy, Debug)]
struct VerletObject {
    radius: f32,

    current_position: Vec2,
    old_position: Vec2,
    acceleration: Vec2,
    colour: Color,
}
impl VerletObject {
    fn update_pos(&mut self, dt: f32) {
        let velocity: Vec2 = self.current_position - self.old_position;
        self.old_position = self.current_position;

        self.current_position = self.current_position + velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2 { x: 0.0, y: 0.0 };
    }

    fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;
    }
}

struct Solver {
    objects: Vec<VerletObject>,
    gravity: Vec2,
}
impl Solver {
    // Core functions
    fn update(&mut self, dt: f32, steps: u32) {
        for _step in 0..steps {
            Self::apply_gravity(self);

            // TODO: REMOVE HARDCODED STUFF
            Self::apply_constraint(self);
            Self::solve_collisions(self);
            Self::update_all_pos(self, dt);
        }
    }

    fn apply_gravity(&mut self) {
        for obj in &mut self.objects {
            obj.accelerate(self.gravity);
        }
    }

    fn update_all_pos(&mut self, dt: f32) {
        for obj in &mut self.objects {
            obj.update_pos(dt);
        }
    }
    fn solve_collisions(&mut self) {
        let object_count = self.objects.len();

        for obj_idx_1 in 0..object_count {
            for obj_idx_2 in (obj_idx_1 + 1)..object_count {
                // Use separate variables to avoid borrowing issues
                let (obj_1_ref, obj_2_ref) = {
                    let (o1, o2) = self.objects.split_at_mut(obj_idx_2);
                    (&mut o1[obj_idx_1], &mut o2[0])
                };

                let collision_axis = obj_1_ref.current_position - obj_2_ref.current_position;
                let dist = collision_axis.length();

                if dist < (obj_1_ref.radius + obj_2_ref.radius) {
                    let n = collision_axis / dist;
                    let delta = (obj_1_ref.radius + obj_2_ref.radius) - dist;
                    let displacement = 0.5 * delta * n;
                    obj_1_ref.current_position += displacement;
                    obj_2_ref.current_position -= displacement;
                }
            }
        }
    }

    // Other functions
    fn apply_constraint(&mut self) {
        for obj in &mut self.objects {
            // Ensure the object's position stays within the screen boundaries
            if obj.current_position.x < 0.0 + obj.radius {
                obj.current_position.x = 0.0 + obj.radius;
            } else if obj.current_position.x > screen_width() - obj.radius {
                obj.current_position.x = screen_width() - obj.radius;
            }

            if obj.current_position.y < 0.0 + obj.radius {
                obj.current_position.y = 0.0 + obj.radius;
            } else if obj.current_position.y > screen_height() - obj.radius {
                obj.current_position.y = screen_height() - obj.radius;
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "GRAVITY".to_owned(),
        fullscreen: true,
        window_height: 1080,
        window_width: 1920,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let center = Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let dt: f32 = 1.0 / 60.0 / 8.0; // Frame rate
    let mut solver = Solver {
        objects: vec![VerletObject {
            radius: 25.0,
            current_position: Vec2 {
                x: center.x,
                y: center.y,
            },
            old_position: Vec2 {
                x: center.x,
                y: center.y,
            },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            colour: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        }],
        gravity: Vec2 { x: 0.0, y: 980.7 }, // Using earth gravity
    };
    loop {
        clear_background(BLACK);

        // Draws physics circles
        for obj in &mut solver.objects {
            draw_circle(
                obj.current_position.x,
                obj.current_position.y,
                obj.radius,
                // Water colours based on speed with foam. If not moving: blue, if moving white.
                Color {
                    r: (obj.current_position.x + obj.current_position.y
                        - obj.old_position.x
                        - obj.old_position.y)
                        * 1.05,
                    g: (obj.current_position.x + obj.current_position.y
                        - obj.old_position.x
                        - obj.old_position.y)
                        * 1.05,
                    b: 1.0,
                    a: 1.0,
                },
            )
        }

        // Creates random circles when mouse down
        if is_mouse_button_down(MouseButton::Left) {
            solver.objects.push(VerletObject {
                radius: rand::gen_range(5.0, 25.0),
                current_position: Vec2 {
                    x: mouse_position().0,
                    y: mouse_position().1,
                },
                old_position: Vec2 {
                    x: mouse_position().0,
                    y: mouse_position().1,
                },
                acceleration: Vec2 { x: 0.0, y: 0.0 },
                colour: Color {
                    r: rand::gen_range(0.0, 1.0),
                    g: rand::gen_range(0.0, 1.0),
                    b: rand::gen_range(0.0, 1.0),
                    a: 1.0,
                },
            })
        }
        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }

        let start = Instant::now();
        solver.update(dt, 8);
        let sim_time = start.elapsed().as_secs_f32();

        draw_text(
            &("FPS: ".to_owned()
                + &get_fps().to_string()
                + " | SIM TIME: "
                + &(sim_time * 1000.0).to_string()),
            5.0,
            15.0,
            20.0,
            WHITE,
        );
        draw_text(
            &("NUMBER OF OBJECTS: ".to_owned() + &solver.objects.len().to_string()),
            5.0,
            35.0,
            20.0,
            WHITE,
        );
        next_frame().await
    }
}
