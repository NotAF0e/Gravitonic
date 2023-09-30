use crate::Options;
use macroquad::prelude::{screen_height, screen_width, Color, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct VerletObject {
    pub radius: f32,

    pub current_position: Vec2,
    pub old_position: Vec2,
    pub acceleration: Vec2,
    pub colour: Color,
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

pub struct Solver {
    pub objects: Vec<VerletObject>,
    pub gravity: Vec2,
}
impl Solver {
    // Core functions
    pub fn update(&mut self, dt: f32, steps: u32, options: &Options) {
        for _step in 0..steps {
            Self::apply_gravity(
                self,
                options.gravity_center,
                Vec2 {
                    x: screen_width() / 2.0,
                    y: screen_height() / 2.0,
                },
                0.07,
            );

            // TODO: REMOVE HARDCODED STUFF
            Self::apply_constraint(self);
            Self::solve_collisions(self);
            Self::update_all_pos(self, dt);
        }
    }
    fn apply_gravity(&mut self, gravity_center: bool, center: Vec2, gravity_strength: f32) {
        if gravity_center {
            for obj in &mut self.objects {
                let gravity_direction = center - obj.current_position;
                let gravity_force = gravity_direction.normalize() * gravity_strength; // Apply gravity strength

                obj.current_position += gravity_force;
            }
        } else {
            for obj in &mut self.objects {
                obj.accelerate(self.gravity);
            }
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
