#[allow(dead_code)]
pub struct FloatingPointRange {
    start: f64,
    end: f64,
    step: f64,
    current: f64,
    next: f64,
}

#[allow(dead_code)]
impl FloatingPointRange {
    pub fn calc_steps_rounded(start: f64, end: f64, step: f64) -> u64 {
        return ((end - start) / step).abs().round() as u64;
    }

    pub fn calc_steps(start: f64, end: f64, step: f64) -> f64 {
        return ((end - start) / step).abs();
    } 

    pub fn new(start: f64, end: f64, step: f64) -> FloatingPointRange {
        return FloatingPointRange {
            start: start,
            end: end,
            step: step,
            current: start - step,
            next: 0.0,
        };
    }
}

impl Iterator for FloatingPointRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current + self.step;
        self.current = next;
        if next <= self.end {
            Some(next)
        } else {
            None
        }
    }
}

