use crate::builder::Builder;

pub struct Wheel3D {
    pub width: f32,
    pub diameter: f32,
    pub offset: raylib::math::Vector3,
}

impl Wheel3D {
    pub fn get_triangle_strip(&self) -> Vec<raylib::math::Vector3> {
        // Creates a triangle strip of the wheel cylinder centered on the
        // origin.
        let mut vertices = Vec::new();
        let radius = self.diameter / 2.0;
        let half_width = self.width / 2.0;
        for i in 0..=20 {
            let angle = 360.0 / 20.0 * (i as f32 * std::f32::consts::PI / 180.0);
            vertices.push(raylib::math::Vector3::new(
                radius * angle.cos(),
                radius * angle.sin(),
                half_width,
            ));
            vertices.push(raylib::math::Vector3::new(
                radius * angle.cos(),
                radius * angle.sin(),
                -half_width,
            ));
        }
        for i in 0..=20 {
            let angle = 360.0 / 20.0 * (i as f32 * std::f32::consts::PI / 180.0);
            vertices.push(raylib::math::Vector3::new(
                radius * angle.cos(),
                radius * angle.sin(),
                -half_width,
            ));
            vertices.push(raylib::math::Vector3::new(
                radius * angle.cos(),
                radius * angle.sin(),
                half_width,
            ));
        }
        vertices
    }
}

pub struct Wheel3DBuilder {
    width: f32,
    diameter: f32,
    offset: Option<raylib::math::Vector3>,
}

impl Wheel3DBuilder {
    pub fn new() -> Self {
        Self {
            width: 0.0,
            diameter: 0.0,
            offset: None,
        }
    }

    pub fn set_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn set_diameter(mut self, diameter: f32) -> Self {
        self.diameter = diameter;
        self
    }

    pub fn set_offset(mut self, offset: raylib::math::Vector3) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Builder<Wheel3D> for Wheel3DBuilder {
    fn create(self) -> Wheel3D {
        Wheel3D {
            width: self.width,
            diameter: self.diameter,
            offset: self.offset.unwrap_or(raylib::math::Vector3::zero()),
        }
    }
}
