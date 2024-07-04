use std::f64::{INFINITY, NEG_INFINITY};

pub fn interval(min: f64, max: f64) -> Interval {
    Interval { min, max }
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

pub const INTERVAL_EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};
pub const INTERVAL_UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};
