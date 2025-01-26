use std::ops::AddAssign;

/// A simple accumulator to calculate mean and variance.
pub struct Accumulator<T> {
    sum: T,
    sum_of_squares: T,
    count: usize,
}

impl<T> Accumulator<T>
where
    T: Default
    + Copy
    + AddAssign<T>
    + std::ops::Div<f64, Output=f64>
    + std::ops::Mul<f64, Output=T>,
{
    /// Creates a new `Accumulator`.
    pub fn new() -> Self {
        Accumulator {
            sum: Default::default(),
            sum_of_squares: Default::default(),
            count: 0,
        }
    }

    /// Returns the mean of the accumulated values.
    pub fn mean(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }

    /// Returns the variance of the accumulated values.
    pub fn variance(&self) -> f64 {
        if self.count <= 1 {
            0.0
        } else {
            let mean = self.sum / self.count as f64;
            (self.sum_of_squares / self.count as f64) - mean * mean
        }
    }
}

impl AddAssign<f64> for Accumulator<f64> {
    fn add_assign(&mut self, rhs: f64) {
        self.sum += rhs;
        self.sum_of_squares += rhs * rhs;
        self.count += 1;
    }
}
