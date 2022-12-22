use is_even::IsEven;

pub struct PositionCalc {
    x0: f32,
    y0: f32,
    pattern_step: f32,
}

impl PositionCalc {
    pub fn new(pattern_size: f32, pattern_offset: f32, x_count: u32, y_count: u32) -> Self {
        assert!(x_count > 0);
        assert!(y_count > 0);

        Self {
            x0: -Self::calc_offset(pattern_size, pattern_offset, x_count),
            y0: -Self::calc_offset(pattern_size, pattern_offset, y_count),
            pattern_step: pattern_size + pattern_offset,
        }
    }

    fn calc_offset(pattern_size: f32, pattern_offset: f32, count: u32) -> f32 {
        if !count.is_even() {
            if count == 1 {
                0.0
            } else {
                (pattern_size + pattern_offset) * ((count - 2) as f32 / 2.0) + pattern_size / 2.0
            }
        } else {
            (pattern_size + pattern_offset) * (count - 1) as f32 / 2.0
        }
    }

    pub fn position(&self, x: usize, y: usize) -> (f32, f32) {
        (
            self.x0 + self.pattern_step * x as f32,
            self.y0 + self.pattern_step * y as f32,
        )
    }
}
