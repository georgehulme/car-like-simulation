use core::f32;

use raylib::prelude::RaylibDraw;

pub(crate) struct Vehicle {
    // Position of the vehicle center
    pub position: raylib::math::Vector2,
    // Speed of the vehicle
    pub speed: f32,
    // Direction of the vehicle
    pub direction: raylib::math::Vector2,
    // Curvature
    pub curvature: f32,
    // Offset of the rotation center, along the vehicle's length, relative to
    // the vehicle's center (positive is towards the vehicle front).
    pub pivot_offset: raylib::math::Vector2,
    // Diameter of the wheels
    pub wheels: Vec<crate::wheel::Wheel>,
}

impl Vehicle {
    pub(crate) fn update(&mut self, dt: f32) {
        // Use euler's method to calculate next step.
        let direction_angle = raylib::math::Vector2::zero().angle_to(self.direction);
        let angle = self.speed * self.curvature * dt;
        let total_velocity = self.direction * self.speed;
        // If
        // x' = x cos a - y sin a, y' = x * sin a + y * cos a, dx = x' - x,
        // dy = y' - y
        // Then
        // dx = x * (cos a - 1) - y * sin a, dy = x * sin a + y * (cos a - 1) 
        let pivot_delta = raylib::math::Vector2::new(
            self.pivot_offset.x * (angle.cos() - 1.0) - self.pivot_offset.y * angle.sin(),
            self.pivot_offset.y * (angle.cos() - 1.0) + self.pivot_offset.x * angle.sin(),
        );
        self.position += total_velocity * dt - pivot_delta.rotated(direction_angle);
        self.direction = self.direction.rotated(angle).normalized();
    }
    
    pub(crate) fn draw(&self, drawer: &mut raylib::drawing::RaylibDrawHandle<'_>) {
        let vehicle_angle = raylib::math::Vector2::zero().angle_to(self.direction);
        let turn_radius = if self.curvature != 0.0 {
            1.0 / self.curvature
        } else {
            f32::INFINITY
        };
        let turn_center = self.position
            + raylib::math::Vector2::new(0.0, 1.0).rotated(vehicle_angle) * turn_radius
            + self.pivot_offset.rotated(vehicle_angle);
        if turn_radius.is_finite() {
            drawer.draw_circle_lines(
                turn_center.x as i32,
                turn_center.y as i32,
                turn_radius,
                raylib::color::Color::RED,
            );
        }
        let mut lines = Vec::new();
        for wheel in &self.wheels {
            let angle = f32::atan2(
                wheel.offset.x - self.pivot_offset.x,
                turn_radius - wheel.offset.y + self.pivot_offset.y,
            );
            if turn_radius.is_finite() {
                lines.push((
                    wheel.offset,
                    (turn_center - self.position).rotated(-vehicle_angle),
                    raylib::color::Color::GREEN,
                ));
            }
            lines.extend(wheel.get_lines(angle));
        }
        lines.iter_mut().for_each(|(start, end, _)| {
            *start = start.rotated(vehicle_angle) + self.position;
            *end = end.rotated(vehicle_angle) + self.position;
        });
        lines.iter().for_each(|(start, end, color)| {
            drawer.draw_line(
                start.x as i32,
                start.y as i32,
                end.x as i32,
                end.y as i32,
                color,
            );
        });
    }
}
