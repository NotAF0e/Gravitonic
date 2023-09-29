use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
struct VerletObject {
    radius: f32,

    position_current: Vec2,
    position_old: Vec2,
    acceleration: Vec2,
}
impl VerletObject {
    fn update_pos(&mut self, dt: f32) {
        let velocity: Vec2 = self.position_current - self.position_old;
        self.position_old = self.position_current;

        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
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
            Self::apply_constraint(self, Vec2 { x: 960.0, y: 540.0 }, 300.0);
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

                let collision_axis = obj_1_ref.position_current - obj_2_ref.position_current;
                let dist = collision_axis.length();

                if dist < (obj_1_ref.radius + obj_2_ref.radius) {
                    let n = collision_axis / dist;
                    let delta = (obj_1_ref.radius + obj_2_ref.radius) - dist;
                    let displacement = 0.5 * delta * n;
                    obj_1_ref.position_current += displacement;
                    obj_2_ref.position_current -= displacement;
                }
            }
        }
    }

    // Other functions
    fn apply_constraint(&mut self, position: Vec2, radius: f32) {
        for obj in &mut self.objects {
            let dist_to_obj: Vec2 = obj.position_current - position;
            let dist: f32 = dist_to_obj.length(); // Assuming you have a length() method for Vec2

            if dist > (radius - obj.radius) {
                // Implement collision response here, e.g., move the object away from the constraint.
                obj.position_current = position + (dist_to_obj / dist) * (radius - obj.radius);
            }
        }
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "GRAVITY".to_owned(),
        fullscreen: false,
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
            position_current: Vec2 {
                x: 1200.0,
                y: 540.0,
            },
            position_old: Vec2 {
                x: 1200.0,
                y: 540.0,
            },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
        }],
        gravity: Vec2 { x: 0.0, y: 980.7 }, // Using earth gravity
    };
    loop {
        clear_background(BLACK);

        // For testing
        draw_circle(960.0, 540.0, 300.0, RED);

        for obj in &mut solver.objects {
            draw_circle(
                obj.position_current.x,
                obj.position_current.y,
                obj.radius,
                WHITE,
            )
        }

        if is_mouse_button_down(MouseButton::Left) {
            solver.objects.push(VerletObject {
                radius: 5.0,
                position_current: Vec2 {
                    x: mouse_position().0,
                    y: mouse_position().1,
                },
                position_old: Vec2 {
                    x: mouse_position().0,
                    y: mouse_position().1,
                },
                acceleration: Vec2 { x: 0.0, y: 0.0 },
            })
        }

        draw_text(
            &("FPS: ".to_owned()
                + &get_fps().to_string()
                + " | FRAME TIME: "
                + &get_frame_time().to_string()),
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

        solver.update(dt, 8);
        next_frame().await
    }
}
