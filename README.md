This is inspired
by [Trading at light speed: designing low latency systems in C++ - David Gross - Meeting C++ 2022 (IS YOUR SYSTEM TUNED CORRECTLY?)](https://www.youtube.com/watch?v=8uAW5FQtcvE&t=2820s)

# RDTSC Timing Analysis

Supporting both x86_64 and Aarch64 via [tick_counter](https://github.com/sheroz/tick_counter) crate.

- x86_64:
  executes [RDTSC](https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/ia-32-ia-64-benchmark-code-execution-paper.pdf)
  CPU instruction to read the time-stamp counter.
- AArch64: reads value of
  the [CNTVCT_EL0](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register)
  counter-timer register.

This project runs the time stamp counter instruction on each core and calculates the mean and standard deviation of the
time stamp counter
call on each core. This should show how well your cores are isolated and creates a simple benchmark to see impact of
your own optimizations.

# Usage

Update these constant fields depending on your core number and samples count you want to have.

```rust
/// Set the desired number of samples
const SAMPLES_COUNT: usize = 65_536;

/// We need to have this number for the isolated cores
/// core_affinity::get_core_ids() cannot get the isolated cores
const NUMBER_OF_CORES: usize = 16;
```

# Output

Core 0 and 1 are not isolated and belong to Linux. However, the other cores are isolated, and it can be seen that
standard
deviation of the isolated cores are very low compared to non isolated cores.

```aiignore
Samples count: 65536
+------+--------------------+--------------------+
| Core | Mean               | Standard Deviation |
+------+--------------------+--------------------+
| 0    | 35.81408407721065  | 102.16968842132331 |
+------+--------------------+--------------------+
| 1    | 35.440451667048144 | 111.69283474492822 |
+------+--------------------+--------------------+
| 2    | 31.550789654383156 | 14.77056952221324  |
+------+--------------------+--------------------+
| 3    | 31.40567635614557  | 14.46332782873708  |
+------+--------------------+--------------------+
| 4    | 31.365743495841915 | 14.526739546567871 |
+------+--------------------+--------------------+
| 5    | 31.521416037232015 | 14.374671442299126 |
+------+--------------------+--------------------+
| 6    | 31.407446402685586 | 14.46354604845707  |
+------+--------------------+--------------------+
| 7    | 31.338124666208895 | 14.516863242597434 |
+------+--------------------+--------------------+
| 8    | 31.338124666208895 | 14.516810685505373 |
+------+--------------------+--------------------+
| 9    | 31.4373998626688   | 14.440285668257943 |
+------+--------------------+--------------------+
| 10   | 31.527885862516214 | 14.45535809403722  |
+------+--------------------+--------------------+
| 11   | 31.520851453421837 | 14.375188170370395 |
+------+--------------------+--------------------+
| 12   | 31.522606240939957 | 14.382999216387143 |
+------+--------------------+--------------------+
| 13   | 31.520851453421837 | 14.373669084353754 |
+------+--------------------+--------------------+
| 14   | 31.52083619439994  | 14.373643627779355 |
+------+--------------------+--------------------+
| 15   | 31.475593194476232 | 14.40908299801948  |
+------+--------------------+--------------------+
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
