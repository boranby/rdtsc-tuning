mod accumulator;
mod rdtsc;

use std::ops::AddAssign;

use core_affinity::CoreId;
use prettytable::{row, Table};

use crate::rdtsc::rdtsc;

/// Set the desired number of samples
const SAMPLES_COUNT: usize = 8192;

/// The main function sets the current thread to run on each core and executes
/// the `run` function.
fn main() {
    let core_ids = core_affinity::get_core_ids().expect(
        "Cannot retrieve information on all the cores on which the current thread is allowed to \
         run.",
    );

    let mut table = Table::new();
    table.add_row(row!["Core", "Mean", "Standard Deviation"]);
    for core_id in core_ids {
        core_affinity::set_for_current(CoreId { id: core_id.id });
        run(&core_id, &mut table);
    }

    table.printstd();
}

/// Runs the benchmarking process by collecting samples and calculating
/// statistics.
#[inline]
fn run(core_id: &CoreId, table: &mut Table) {
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

    table.add_row(row![core_id.id, mean, std_dev]);
}
