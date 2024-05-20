use criterion::{Criterion, criterion_group, criterion_main};

use tick_counter::aarch64_tick_counter;

#[inline]
pub fn rdtsc() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    return x86_64_tick_counter();
    #[cfg(target_arch = "aarch64")]
    return aarch64_tick_counter();
}

fn bench_rdtsc(c: &mut Criterion) {
    c.bench_function("rdtsc", |b| b.iter(|| rdtsc()));
}

fn bench_instant_now(c: &mut Criterion) {
    c.bench_function("instant_now", |b| b.iter(|| std::time::Instant::now()));
}

criterion_group!(benches, bench_rdtsc, bench_instant_now);
criterion_main!(benches);
