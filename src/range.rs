use std::num::ParseFloatError;

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub from: f32,
    pub to: f32,
    pub step: f32,
}

impl Range {
    pub fn count(&self) -> usize {
        ((self.to - self.from) / self.step).ceil() as usize + 1
    }

    pub fn into_iter(self) -> RangeItrator {
        RangeItrator::new(self)
    }
}

pub struct RangeItrator {
    range: Range,
    current: f32,
}

impl RangeItrator {
    pub fn new(range: Range) -> Self {
        Self {
            current: range.from,
            range,
        }
    }
}

impl Iterator for RangeItrator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let curent = self.current;
        if curent <= self.range.to {
            self.current = curent + self.range.step;
            Some(curent)
        } else {
            None
        }
    }
}

pub fn parse_range(src: &str) -> Result<Range, ParseFloatError> {
    let mut elements = src.split(':');

    let from: f32 = if let Some(v) = elements.next() {
        v.parse()?
    } else {
        src.parse()?
    };
    let to: f32 = elements
        .next()
        .unwrap_or(from.to_string().as_str())
        .parse()?;
    let step: f32 = elements.next().unwrap_or("1").parse()?;

    Ok(Range { from, to, step })
}
