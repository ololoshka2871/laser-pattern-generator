pub struct SquarePattern {
    pos: (f32, f32),
    s: f32,
    f: f32,
    a: f32,
    laser_height: f32,
    half_size: f32,
}

impl SquarePattern {
    pub fn new(pos: (f32, f32), size: f32, s: f32, f: f32, a: f32, laser_height: f32) -> Self {
        Self {
            pos,
            s,
            f,
            a,
            laser_height,
            half_size: size / 2.0,
        }
    }

    pub fn generate(&self) -> String {
        use std::io::Write;

        let mut output: Vec<u8> = vec![];

        writeln!(
            output,
            "(Test pattern at {:?} S={}, F={})",
            self.pos, self.s, self.f
        )
        .unwrap();

        // prepare
        writeln!(
            output,
            "G0 X{}Y{}A{}
M3S{}",
            self.pos.0, self.pos.1, self.a, self.s,
        )
        .unwrap();

        // ^---->
        // |....|
        // |.*->|
        // |...||
        // <---||

        let (mut current_x, mut current_y) = self.pos;
        let mut step: f32 = 1.0;

        let x_limits = (self.pos.0 + self.half_size, self.pos.0 - self.half_size);
        let y_limits = (self.pos.1 + self.half_size, self.pos.1 - self.half_size);

        loop {
            current_x += self.laser_height * step;
            if current_x > x_limits.0 || current_x < x_limits.1 {
                break;
            }
            writeln!(output, "G1 X{}F{}", current_x, self.f).unwrap();

            current_y += self.laser_height * step;
            if current_y > y_limits.0 || current_y < y_limits.1 {
                break;
            }
            writeln!(output, "Y{}", current_y).unwrap();

            step = (step.abs() + 1.0) * if step > 0.0 { -1.0 } else { 1.0 };
        }

        // end
        writeln!(output, "M5").unwrap();

        String::from_utf8(output).unwrap()
    }
}
