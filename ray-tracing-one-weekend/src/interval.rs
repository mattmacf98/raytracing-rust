#[derive(Clone, Copy, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval {
            min,
            max
        }
    }

    pub fn from_sub_intervals(a: Interval, b: Interval) -> Interval {
        let min = f64::min(a.min, b.min);
        let max = f64::max(a.max, b.max);

        Interval {
            min,
            max
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta/2.0;
        Interval::new(self.min - padding, self.max + padding)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}