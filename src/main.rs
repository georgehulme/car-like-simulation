use raylib::prelude::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use raylib::prelude::*;
use std::f32::consts::TAU;
use vehicle::Vehicle;
use wheel::Wheel;

mod vehicle;
mod wheel;

fn main() {
    let (mut rl, thread) = raylib::init().title("Car like kinematics").build();

    let (width, height) = (rl.get_render_width(), rl.get_screen_height());

    rl.set_target_fps(60);

    let wheel_base: f32 = 50.0;
    let wheel_track: f32 = 30.0;

    let wheel_width: f32 = 8.0;
    let wheel_diameter: f32 = 20.0;

    let mut vehicle = Vehicle {
        position: Vector2 { x: 100.0, y: 100.0 },
        speed: 0.0,
        direction: Vector2::new(1.0, 0.0),
        curvature: 0.0,
        pivot_offset: raylib::math::Vector2::new(-wheel_base / 2.0, 0.0),
        wheels: vec![
            // Back left
            Wheel {
                offset: Vector2::new(-wheel_base / 2.0, -wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Back right
            Wheel {
                offset: Vector2::new(-wheel_base / 2.0, wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Front left
            Wheel {
                offset: Vector2::new(wheel_base / 2.0, -wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Front right
            Wheel {
                offset: Vector2::new(wheel_base / 2.0, wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
        ],
    };

    while !rl.window_should_close() {
        // Update with semi-fixed timestep
        vehicle.update(f32::min(1.0 / 32.0, rl.get_frame_time()));

        // Clamp to window nicely
        vehicle.position.x = f32::rem_euclid(vehicle.position.x, width as f32);
        vehicle.position.y = f32::rem_euclid(vehicle.position.y, height as f32);

        // Input
        if rl.is_key_down(KEY_RIGHT) {
            vehicle.curvature += TAU / wheel_base * 0.01;
        }
        if rl.is_key_down(KEY_LEFT) {
            vehicle.curvature -= TAU / wheel_base * 0.01;
        }
        if rl.is_key_down(KEY_UP) {
            vehicle.speed += 1.0;
        }
        if rl.is_key_down(KEY_DOWN) {
            vehicle.speed -= 1.0;
        }
        vehicle.curvature = vehicle
            .curvature
            .clamp(-TAU / (8.0 * wheel_base), TAU / (8.0 * wheel_base));
        vehicle.speed = vehicle.speed.clamp(-500.0, 500.0);

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(
            format!("Speed: {}", vehicle.speed).as_str(),
            12,
            12,
            20,
            Color::BLACK,
        );
        d.draw_text(
            format!("Anglular Velocity: {}", vehicle.curvature * vehicle.speed).as_str(),
            12,
            32,
            20,
            Color::BLACK,
        );
        d.draw_text(
            "Move with arrows!".to_string().as_str(),
            12,
            height - 24,
            20,
            Color::GRAY,
        );
        vehicle.draw(&mut d);
    }
}
