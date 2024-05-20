
use std::ops::AddAssign;

#[cfg(target_arch = "aarch64")]
use tick_counter::aarch64_tick_counter;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use tick_counter::x86_64_tick_counter;

fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    // for each core ids
    for core_id in core_ids {
        core_affinity::set_for_current(core_id);
        println!("Core {:?} is running", core_id);
        run();
        println!();
    }
}

#[inline]
fn run() {
    const SAMPLES_COUNT: usize = 1024; // Set the desired number of samples

    let mut samples: Vec<u64> = vec![0; SAMPLES_COUNT];
    for s in &mut samples {
        *s = rdtsc();
    }

    let mut c = accumulator::Accumulator::<f64>::new();
    for i in 1..samples.len() {
        c.add_assign(samples[i] as f64 - samples[i - 1] as f64);
    }

    let mean = c.mean();
    let std_dev = (c.variance() * samples.len() as f64 / (samples.len() - 1) as f64).sqrt();

    println!("Mean: {}", mean);
    println!("Standard Deviation: {}", std_dev);
}

mod accumulator {
    use std::ops::AddAssign;

    pub struct Accumulator<T> {
        sum: T,
        sum_of_squares: T,
        count: usize,
    }

    impl<T> Accumulator<T>
        where
            T: Default + Copy + AddAssign<T> + std::ops::Div<f64, Output=f64> + std::ops::Mul<f64, Output=T>
    {
        pub fn new() -> Self {
            Accumulator {
                sum: Default::default(),
                sum_of_squares: Default::default(),
                count: 0,
            }
        }

        pub fn mean(&self) -> f64 {
            if self.count == 0 {
                0.0
            } else {
                self.sum / self.count as f64
            }
        }

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
}

#[inline]
fn rdtsc() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    return x86_64_tick_counter();
    #[cfg(target_arch = "aarch64")]
    return aarch64_tick_counter();
}
