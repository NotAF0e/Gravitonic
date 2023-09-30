use crate::solver;
use crate::Options;
use macroquad::prelude::*;
use std::process::exit;

pub fn handle_input(solver: &mut solver::Solver, options: &mut Options) {
    // Creates random circles when mouse down
    if is_mouse_button_down(MouseButton::Left) {
        solver.objects.push(solver::VerletObject {
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
    if is_key_pressed(KeyCode::G) && options.gravity_center {
        options.gravity_center = false;
    } else if is_key_pressed(KeyCode::G) && !options.gravity_center {
        options.gravity_center = true;
    }
}
