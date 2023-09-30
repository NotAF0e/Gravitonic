use macroquad::prelude::*;
use std::time::Instant;
mod input;
mod solver;

pub struct Options {
    gravity_center: bool,
}

// Configurations for the app
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
    let dt: f32 = 1.0 / 60.0 / 8.0; // Frame rate
    let mut solver = solver::Solver {
        objects: vec![],
        gravity: Vec2 { x: 0.0, y: 980.7 }, // Using earth gravity
    };

    let _center = Vec2 {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
    };
    let mut options = Options {
        gravity_center: false,
    };

    // Main loop
    loop {
        clear_background(BLACK);

        // Draws physics circles
        for obj in &mut solver.objects {
            draw_circle(
                obj.current_position.x,
                obj.current_position.y,
                obj.radius,
                // Revered colors to gray for planet color
                Color {
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 1.0,
                },
            )
        }

        input::handle_input(&mut solver, &mut options);

        let start = Instant::now();
        solver.update(dt, 8, &options);
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
