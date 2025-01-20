pub(crate) struct Wheel {
    pub width: f32,
    pub diameter: f32,
    pub offset: raylib::math::Vector2,
}

impl Wheel {
    pub fn get_lines(
        &self,
        wheel_angle: f32,
    ) -> Vec<(
        raylib::math::Vector2,
        raylib::math::Vector2,
        raylib::color::Color,
    )> {
        // Calculate the wheel rectangle
        let mut rect = [
            raylib::math::Vector2::new(-self.diameter / 2.0, -self.width / 2.0),
            raylib::math::Vector2::new(self.diameter / 2.0, -self.width / 2.0),
            raylib::math::Vector2::new(self.diameter / 2.0, self.width / 2.0),
            raylib::math::Vector2::new(-self.diameter / 2.0, self.width / 2.0),
        ];
        rect.iter_mut()
            .for_each(|r| *r = r.rotated(wheel_angle) + self.offset);

        // Calculate the lines of the rectangle
        let mut lines = Vec::new();
        for i in 0..4 {
            let j = (i + 1) % 4;
            lines.push((rect[i], rect[j], raylib::color::Color::BLACK));
        }
        lines
    }
}
