use core::f32;
use raylib::prelude::{RaylibDraw3D, RaylibMode3DExt};
use crate::builder::Builder;

const DEBUG: bool = true;

pub(crate) struct Vehicle3D {
    // Position of the vehicle center
    pub position: raylib::math::Vector3,
    // Speed of the vehicle
    pub speed: f32,
    // Direction of the vehicle
    pub direction: raylib::math::Vector3,
    // Curvature
    pub curvature: f32,
    // Vehicle_angle
    pub vehicle_angle: f32,
    // Offset of the rotation center, along the vehicle's length, relative to
    // the vehicle's center (positive is towards the vehicle front).
    pub pivot_offset: raylib::math::Vector3,
    // Diameter of the wheels
    pub wheels: Vec<crate::wheel::Wheel3D>,
}

pub struct Vehicle3DBuilder {
    position: Option<raylib::math::Vector3>,
    speed: f32,
    direction: Option<raylib::math::Vector3>,
    pivot_offset: Option<raylib::math::Vector3>,
    wheels: Vec<crate::wheel::Wheel3D>,
}

impl Vehicle3DBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            speed: 0.0,
            direction: None,
            pivot_offset: None,
            wheels: Vec::new(),
        }
    }

    pub fn set_position(mut self, pos: raylib::math::Vector3) -> Self {
        self.position = Some(pos);
        self
    }

    pub fn set_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn set_direction(mut self, dir: raylib::math::Vector3) -> Self {
        self.direction = Some(dir.normalized());
        self
    }

    pub fn set_pivot_offset(mut self, offset: raylib::math::Vector3) -> Self {
        self.pivot_offset = Some(offset);
        self
    }

    pub fn add_wheel(mut self, wheel: crate::wheel::Wheel3D) -> Self {
        self.wheels.push(wheel);
        self
    }
}

impl Builder<Vehicle3D> for Vehicle3DBuilder {
    fn create(self) -> Vehicle3D {
        Vehicle3D {
            position: self.position.unwrap_or(raylib::math::Vector3::zero()),
            speed: self.speed,
            direction: self.direction.unwrap_or(raylib::math::Vector3::new(1.0, 0.0, 0.0)),
            curvature: 0.0,
            vehicle_angle: 0.0,
            pivot_offset: self.pivot_offset.unwrap_or(raylib::math::Vector3::zero()),
            wheels: self.wheels,
        }
    }
}

impl Vehicle3D {
    pub(crate) fn update(&mut self, dt: f32) {
        // Use euler's method to calculate next step.
        let angle = self.speed * self.curvature * dt;
        let total_velocity = self.direction * self.speed;
        // If
        // x' = x cos a - z sin a, z' = x * sin a + z * cos a, dx = x' - x,
        // dz = z' - z
        // Then
        // dx = x * (cos a - 1) - z * sin a, dz = x * sin a + z * (cos a - 1)
        let (s, c) = angle.sin_cos();
        let mut pivot_delta = raylib::math::Vector3::new(
            self.pivot_offset.x * (c - 1.0) - self.pivot_offset.z * s,
            0.0,
            self.pivot_offset.z * (c - 1.0) + self.pivot_offset.x * s,
        );
        pivot_delta.rotate(raylib::math::Quaternion::from_axis_angle(
            raylib::math::Vector3::up(),
            self.vehicle_angle,
        ));
        self.position += total_velocity * dt + pivot_delta;
        self.direction
            .rotate(raylib::math::Quaternion::from_axis_angle(
                raylib::math::Vector3::up(),
                angle,
            ));
        self.direction = self.direction.normalized();
        self.vehicle_angle += angle;
        self.vehicle_angle = self.vehicle_angle.rem_euclid(2.0 * std::f32::consts::PI);
    }

    pub(crate) fn draw(
        &self,
        camera: raylib::camera::Camera3D,
        drawer: &mut raylib::drawing::RaylibMode3D<'_, raylib::drawing::RaylibDrawHandle<'_>>,
    ) {
        let turn_radius = if self.curvature != 0.0 {
            1.0 / self.curvature
        } else {
            f32::INFINITY
        };
        let mut drawer_3d = drawer.begin_mode3D(camera);
        let turn_center = self.position
            + (raylib::math::Vector3::new(
                self.pivot_offset.x,
                self.pivot_offset.y,
                self.pivot_offset.z,
            ) - raylib::math::Vector3::new(0.0, 0.0, turn_radius))
            .rotate_by(raylib::math::Quaternion::from_axis_angle(
                raylib::math::Vector3::up(),
                self.vehicle_angle,
            ));
        if DEBUG && turn_radius.is_finite() {
            drawer_3d.draw_sphere(
                turn_center - raylib::math::Vector3::up() * self.position.y,
                0.2,
                raylib::color::Color::GREEN,
            );
        }
        for wheel in &self.wheels {
            let angle = f32::atan2(
                wheel.offset.x - self.pivot_offset.x,
                turn_radius - wheel.offset.y + self.pivot_offset.y,
            );
            let mut wheel_tri_strip = wheel.get_triangle_strip();
            wheel_tri_strip.iter_mut().for_each(|v| {
                v.rotate(raylib::math::Quaternion::from_axis_angle(
                    raylib::math::Vector3::up(),
                    angle,
                ));
                *v += wheel.offset;
                v.rotate(raylib::math::Quaternion::from_axis_angle(
                    raylib::math::Vector3::up(),
                    self.vehicle_angle,
                ));
                *v += self.position;
            });
            drawer_3d
                .draw_triangle_strip3D(wheel_tri_strip.as_slice(), raylib::color::Color::BLACK);
            if DEBUG {
                if turn_radius.is_finite() {
                    drawer_3d.draw_line_3D(
                        self.position
                            + wheel
                                .offset
                                .rotate_by(raylib::math::Quaternion::from_axis_angle(
                                    raylib::math::Vector3::up(),
                                    self.vehicle_angle,
                                )),
                        turn_center - raylib::math::Vector3::up() * self.position.y,
                        raylib::color::Color::RED,
                    );
                } else {
                    drawer_3d.draw_line_3D(
                        self.position
                            + (wheel.offset - raylib::math::Vector3::right()).rotate_by(
                                raylib::math::Quaternion::from_axis_angle(
                                    raylib::math::Vector3::up(),
                                    self.vehicle_angle,
                                ),
                            ),
                        self.position
                            + (wheel.offset + raylib::math::Vector3::right()).rotate_by(
                                raylib::math::Quaternion::from_axis_angle(
                                    raylib::math::Vector3::up(),
                                    self.vehicle_angle,
                                ),
                            ),
                        raylib::color::Color::RED,
                    );
                }
            }
        }
    }
}
