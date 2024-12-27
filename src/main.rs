use std::f32::consts::TAU;
use raylib::prelude::*;
use raylib::prelude::KeyboardKey::{KEY_RIGHT,KEY_DOWN,KEY_UP,KEY_LEFT};

fn main() {
    let width = 640;
    let height = 480;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Car like kinematics")
        .build();

    rl.set_target_fps(60);

    let wheel_base: f32 = 50.0;
    let wheel_track: f32 = 30.0;

    let wheel_width: f32 = 8.0;
    let wheel_diameter: f32 = 20.0;

    let mut vehicle_center = Vector2 { x: 100.0, y: 100.0 };
    let mut vehicle_angle: f32 = 0.0;

    let mut car_like_velocity: f32 = 0.0;
    let mut car_like_angle: f32 = 0.0;

    while !rl.window_should_close() {
        // Update
        // Use euler to integrate
        // x' = car_like_velocity cos ψ
        // y' = car_like_velocity sin ψ
        // ψ' = (car_like_velocity / L) tan(-car_like_angle)
        let dt = rl.get_frame_time();
        vehicle_center.x += car_like_velocity * f32::cos(vehicle_angle) * dt;
        vehicle_center.y += car_like_velocity * f32::sin(vehicle_angle) * dt;
        vehicle_angle += car_like_velocity / wheel_base * f32::tan(-car_like_angle) * dt;

        // Clamp to window nicely
        vehicle_center.x = f32::rem_euclid(vehicle_center.x, width as f32);
        vehicle_center.y = f32::rem_euclid(vehicle_center.y, height as f32);

        // Input
        if rl.is_key_down(KEY_RIGHT) {
            car_like_angle = f32::max(car_like_angle - TAU * 0.01, -TAU / 8.0);
        }
        if rl.is_key_down(KEY_LEFT) {
            car_like_angle = f32::min(car_like_angle + TAU * 0.01, TAU / 8.0);
        }
        if rl.is_key_down(KEY_UP) {
            car_like_velocity = f32::min(car_like_velocity + 1.0, 500.0);
        }
        if rl.is_key_down(KEY_DOWN) {
            car_like_velocity = f32::max(car_like_velocity - 1.0, -500.0);
        }

        // Precalculate
        let left_angle = f32::atan(wheel_base / (wheel_base / f32::tan(-car_like_angle) + wheel_track / 2.0));
        let right_angle = f32::atan(wheel_base / (wheel_base / f32::tan(-car_like_angle) - wheel_track / 2.0));

        // Draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(format!("Velocity: {}", car_like_velocity).as_str(), 12, 12, 20, Color::BLACK);
        d.draw_text(format!("Angle: {}", car_like_angle).as_str(),12, 32, 20, Color::BLACK);
        d.draw_text("Move with arrows!".to_string().as_str(), 12, height - 24, 20, Color::GRAY);

        unsafe { ffi::rlPushMatrix(); }
            unsafe { ffi::rlTranslatef(vehicle_center.x, vehicle_center.y, 0.0); }
            unsafe { ffi::rlRotatef(vehicle_angle.to_degrees(), 0.0, 0.0, 1.0); }

            unsafe { ffi::rlPushMatrix(); }
                unsafe { ffi::rlTranslatef(0.0, -wheel_track/2.0, 0.0); }
                d.draw_rectangle_lines((-wheel_diameter / 2.0) as i32, (-wheel_width / 2.0) as i32,
                                       wheel_diameter as i32, wheel_width as i32, Color::BLACK);
            unsafe { ffi::rlPopMatrix(); }


            unsafe { ffi::rlPushMatrix(); }
                unsafe { ffi::rlTranslatef(0.0, wheel_track/2.0, 0.0); }
                d.draw_rectangle_lines((-wheel_diameter / 2.0) as i32, (-wheel_width / 2.0) as i32,
                                       wheel_diameter as i32, wheel_width as i32, Color::BLACK);
            unsafe { ffi::rlPopMatrix(); }

            unsafe { ffi::rlPushMatrix(); }
                unsafe { ffi::rlTranslatef(wheel_base, 0.0, 0.0); }

                unsafe { ffi::rlPushMatrix(); }
                    unsafe { ffi::rlTranslatef(0.0, -wheel_track/2.0, 0.0); }
                    unsafe { ffi::rlRotatef(left_angle.to_degrees(), 0.0, 0.0, 1.0); }
                    d.draw_rectangle_lines((-wheel_diameter / 2.0) as i32, (-wheel_width / 2.0) as i32,
                                           wheel_diameter as i32, wheel_width as i32, Color::BLACK);
                unsafe { ffi::rlPopMatrix(); }


                unsafe { ffi::rlPushMatrix(); }
                    unsafe { ffi::rlTranslatef(0.0, wheel_track/2.0, 0.0); }
                    unsafe { ffi::rlRotatef(right_angle.to_degrees(), 0.0, 0.0, 1.0); }
                    d.draw_rectangle_lines((-wheel_diameter / 2.0) as i32, (-wheel_width / 2.0) as i32,
                                           wheel_diameter as i32, wheel_width as i32, Color::BLACK);
                unsafe { ffi::rlPopMatrix(); }

            unsafe { ffi::rlPopMatrix(); }

        unsafe { ffi::rlPopMatrix(); }
    }
}