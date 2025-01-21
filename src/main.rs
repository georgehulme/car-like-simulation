use raylib::prelude::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use raylib::prelude::*;
use vehicle::Vehicle;
use wheel::Wheel;

mod vehicle;
mod wheel;

const STEERING_RATE: f32 = 2.5 * std::f32::consts::TAU;
const MAX_STEERING_ANGLE: f32 = 1.5 * std::f32::consts::TAU;
const STEERING_RATIO: f32 = 8.0;

const STEER_DAMPING_FACTOR: f32 = 0.3;
const SPEED_DAMPING_FACTOR: f32 = (MAX_SPEED - ACCELERATION * 0.9) / MAX_SPEED;

const ACCELERATION: f32 = 100.0;
const MAX_SPEED: f32 = 500.0;

fn get_turn_curvature(steering_angle: f32, effective_wheel_base: f32) -> f32 {
    f32::sin(steering_angle / STEERING_RATIO) / effective_wheel_base
}

fn main() {
    let (mut rl, thread) = raylib::init().title("Car like kinematics").build();

    let (width, height) = (rl.get_render_width(), rl.get_screen_height());

    rl.set_target_fps(60);

    let effective_wheel_base: f32 = 50.0;
    let effective_wheel_track: f32 = 30.0;

    let wheel_width: f32 = 8.0;
    let wheel_diameter: f32 = 20.0;

    let mut steering_angle = 0.0;

    let mut vehicle = Vehicle {
        position: Vector2 { x: 100.0, y: 100.0 },
        speed: 0.0,
        direction: Vector2::new(1.0, 0.0),
        curvature: get_turn_curvature(steering_angle, effective_wheel_base),
        pivot_offset: raylib::math::Vector2::new(-effective_wheel_base / 2.0, 0.0),
        wheels: vec![
            // Back left
            Wheel {
                offset: Vector2::new(-effective_wheel_base / 2.0, -effective_wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Back right
            Wheel {
                offset: Vector2::new(-effective_wheel_base / 2.0, effective_wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Front left
            Wheel {
                offset: Vector2::new(effective_wheel_base / 2.0, -effective_wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
            // Front right
            Wheel {
                offset: Vector2::new(effective_wheel_base / 2.0, effective_wheel_track / 2.0),
                diameter: wheel_diameter,
                width: wheel_width,
            },
        ],
    };

    while !rl.window_should_close() {
        // Update with semi-fixed timestep
        let dt = f32::min(1.0 / 32.0, rl.get_frame_time());
        vehicle.update(dt);

        // Clamp to window nicely
        vehicle.position.x = f32::rem_euclid(vehicle.position.x, width as f32);
        vehicle.position.y = f32::rem_euclid(vehicle.position.y, height as f32);

        // Damping
        steering_angle *= (STEER_DAMPING_FACTOR).powf(dt * (vehicle.speed / MAX_SPEED));
        steering_angle = (steering_angle * 10000.0).floor() / 10000.0;
        vehicle.speed *= SPEED_DAMPING_FACTOR.powf(dt);
        vehicle.speed = (vehicle.speed * 10000.0).floor() / 10000.0;

        // Input
        if rl.is_key_down(KEY_RIGHT) {
            steering_angle += STEERING_RATE * dt;
        }
        if rl.is_key_down(KEY_LEFT) {
            steering_angle -= STEERING_RATE * dt;
        }
        if rl.is_key_down(KEY_UP) {
            vehicle.speed += ACCELERATION * dt;
        }
        if rl.is_key_down(KEY_DOWN) {
            vehicle.speed -= ACCELERATION * dt;
        }
        steering_angle = steering_angle.clamp(-MAX_STEERING_ANGLE, MAX_STEERING_ANGLE);
        vehicle.speed = vehicle.speed.clamp(-MAX_SPEED, MAX_SPEED);

        vehicle.curvature = get_turn_curvature(steering_angle, effective_wheel_base);

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
        d.draw_text(format!("Steering Angle: {}", (f32::to_degrees(steering_angle) * 100.0).floor() / 100.0).as_str(), 12, 32, 20, Color::BLACK);
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
