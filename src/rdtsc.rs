#[cfg(target_arch = "aarch64")]
use tick_counter::aarch64_tick_counter;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use tick_counter::x86_64_tick_counter;

/// Returns the current value of the time-stamp counter.
#[inline]
pub fn rdtsc() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    return x86_64_tick_counter();
    #[cfg(target_arch = "aarch64")]
    return aarch64_tick_counter();
}