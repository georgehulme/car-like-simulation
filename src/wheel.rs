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
