mod accumulator;
mod rdtsc;

use std::ops::AddAssign;

use core_affinity::CoreId;
use prettytable::{row, Table};

use crate::rdtsc::rdtsc;

/// Set the desired number of samples
const SAMPLES_COUNT: usize = 8192;

/// We need to have this number for the isolated cores
/// core_affinity::get_core_ids() cannot get the isolated cores
const NUMBER_OF_CORES: usize = 16;

fn main() {
    let mut table = Table::new();
    table.add_row(row!["Core", "Mean", "Standard Deviation"]);
    for core_id in 0..NUMBER_OF_CORES {
        core_affinity::set_for_current(CoreId { id: core_id });
        run(&core_id, &mut table);
    }

    table.printstd();
}

#[inline]
fn run(core_id: &usize, table: &mut Table) {
    let mut samples: Vec<u64> = vec![0; SAMPLES_COUNT];
    for s in &mut samples {
        *s = rdtsc();
    }

    let mut accumulator = accumulator::Accumulator::<f64>::new();
    for i in 1..samples.len() {
        accumulator.add_assign(samples[i] as f64 - samples[i - 1] as f64);
    }

    let mean = accumulator.mean();
    let std_dev =
        (accumulator.variance() * samples.len() as f64 / (samples.len() - 1) as f64).sqrt();

    table.add_row(row![core_id, mean, std_dev]);
}
