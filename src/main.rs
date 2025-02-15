use raylib::prelude::KeyboardKey::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use raylib::prelude::*;
use builder::Builder;

mod builder;
mod vehicle;
mod wheel;

const STEERING_RATE: f32 = 1.5 * std::f32::consts::TAU;
const MAX_STEERING_ANGLE: f32 = 1.5 * std::f32::consts::TAU;
const STEERING_RATIO: f32 = 8.0;

const STEER_DAMPING_FACTOR: f32 = 0.3;
const SPEED_DAMPING_FACTOR: f32 = (MAX_SPEED - ACCELERATION * 0.9) / MAX_SPEED;

const ACCELERATION: f32 = 6.0;
const REVERSE_ACCELERATION: f32 = 4.0;
const BRAKING: f32 = 10.0;
const MAX_SPEED: f32 = 50.0;
const MAX_REVERSE_SPEED: f32 = -10.0;

const GRID_SPACING: f32 = 2.0;
const GRID_RESOLUTION: i32 = 1000;
const GRID_SIZE: f32 = GRID_SPACING * GRID_RESOLUTION as f32;

fn get_turn_curvature(steering_angle: f32, effective_wheel_base: f32) -> f32 {
    f32::sin(steering_angle / STEERING_RATIO) / effective_wheel_base
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .resizable()
        .title("Car like kinematics")
        .build();

    let height = rl.get_screen_height();

    rl.set_target_fps(60);

    let effective_wheel_base: f32 = 10.0;
    let effective_wheel_track: f32 = 6.0;

    let wheel_width: f32 = 0.8;
    let wheel_diameter: f32 = 2.0;

    let camera_offset = Vector3::new(-20.0, 20.0, 0.0);

    let mut steering_angle = 0.0;

    // Create wheels using the builder pattern.
    let wheel_bl = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            -effective_wheel_base / 2.0,
            0.0,
            -effective_wheel_track / 2.0,
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();
    let wheel_br = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            -effective_wheel_base / 2.0,
            0.0,
            effective_wheel_track / 2.0,
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();
    let wheel_ml = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            0.0,
            0.0,
            -(effective_wheel_track / 2.0 + wheel_width / 2.0),
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();
    let wheel_mr = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            0.0,
            0.0,
            effective_wheel_track / 2.0 + wheel_width / 2.0,
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();
    let wheel_fl = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            effective_wheel_base / 2.0,
            0.0,
            -effective_wheel_track / 2.0,
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();
    let wheel_fr = wheel::Wheel3DBuilder::new()
        .set_offset(Vector3::new(
            effective_wheel_base / 2.0,
            0.0,
            effective_wheel_track / 2.0,
        ))
        .set_diameter(wheel_diameter)
        .set_width(wheel_width)
        .create();

    // Create the vehicle using the builder pattern.
    let mut vehicle = vehicle::Vehicle3DBuilder::new()
        .set_position(Vector3::new(0.0, wheel_diameter / 2.0, 0.0))
        .set_speed(0.0)
        .set_direction(Vector3::new(1.0, 0.0, 0.0))
        .set_pivot_offset(Vector3::new(-effective_wheel_base / 2.0, 0.0, 0.0))
        .add_wheel(wheel_bl)
        .add_wheel(wheel_br)
        .add_wheel(wheel_ml)
        .add_wheel(wheel_mr)
        .add_wheel(wheel_fl)
        .add_wheel(wheel_fr)
        .create();

    let mut camera = raylib::camera::Camera3D::perspective(
        vehicle.position + camera_offset,
        raylib::math::Vector3::zero(),
        raylib::math::Vector3::up(),
        45.0,
    );

    while !rl.window_should_close() {
        // Update with semi-fixed timestep
        let dt = f32::min(1.0 / 32.0, rl.get_frame_time());
        vehicle.update(dt);
        if vehicle.position.x > GRID_SIZE / 2.0 {
            vehicle.position.x -= GRID_SIZE;
        } else if vehicle.position.x < -GRID_SIZE / 2.0 {
            vehicle.position.x += GRID_SIZE;
        }
        if vehicle.position.z > GRID_SIZE / 2.0 {
            vehicle.position.z -= GRID_SIZE;
        } else if vehicle.position.z < -GRID_SIZE / 2.0 {
            vehicle.position.z += GRID_SIZE;
        }
        camera.position = vehicle.position
            + camera_offset.rotate_by(Quaternion::from_axis_angle(
                Vector3::up(),
                vehicle.vehicle_angle,
            ));
        camera.target = vehicle.position;

        // Damping
        steering_angle *= (STEER_DAMPING_FACTOR).powf(dt);
        steering_angle = (steering_angle * 10000.0).floor() / 10000.0;
        vehicle.speed *= SPEED_DAMPING_FACTOR.powf(dt);
        vehicle.speed = (vehicle.speed * 10000.0).floor() / 10000.0;

        // Input
        if rl.is_key_down(KEY_RIGHT) {
            steering_angle -= STEERING_RATE * dt;
        }
        if rl.is_key_down(KEY_LEFT) {
            steering_angle += STEERING_RATE * dt;
        }
        if rl.is_key_pressed(KEY_UP) || vehicle.speed != 0.0 && rl.is_key_down(KEY_UP) {
            if vehicle.speed < 0.0 {
                vehicle.speed += BRAKING * dt;
                vehicle.speed = vehicle.speed.min(0.0);
            } else {
                vehicle.speed += ACCELERATION * dt;
            }
        }
        if rl.is_key_pressed(KEY_DOWN) || vehicle.speed != 0.0 && rl.is_key_down(KEY_DOWN) {
            if vehicle.speed > 0.0 {
                vehicle.speed -= BRAKING * dt;
                vehicle.speed = vehicle.speed.max(0.0);
            } else {
                vehicle.speed -= REVERSE_ACCELERATION * dt;
            }
        }
        steering_angle = steering_angle.clamp(-MAX_STEERING_ANGLE, MAX_STEERING_ANGLE);
        vehicle.speed = vehicle.speed.clamp(MAX_REVERSE_SPEED, MAX_SPEED);

        vehicle.curvature = get_turn_curvature(steering_angle, effective_wheel_base);

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        {
            let mut d_3d = d.begin_mode3D(camera);
            d_3d.draw_grid(GRID_RESOLUTION, GRID_SPACING);
            vehicle.draw(camera, &mut d_3d);
            d_3d.draw_line_3D(Vector3::zero(), Vector3::up(), Color::RED);
            d_3d.draw_line_3D(Vector3::zero(), Vector3::forward(), Color::GREEN);
            d_3d.draw_line_3D(Vector3::zero(), Vector3::right(), Color::BLUE);
        }
        let mut i = 0;
        d.draw_text(
            &format!("Speed: {}", vehicle.speed),
            12,
            12 + 20 * i,
            20,
            Color::BLACK,
        );
        i += 1;
        d.draw_text(
            &format!(
                "Steering Angle: {}",
                (f32::to_degrees(steering_angle) * 100.0).floor() / 100.0
            ),
            12,
            12 + 20 * i,
            20,
            Color::BLACK,
        );
        d.draw_text("Move with arrows!", 12, height - 24, 20, Color::GRAY);
    }
}
