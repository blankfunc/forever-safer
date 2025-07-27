## AtomicPoll
This benchmark test tests from three aspects: `self increasing time`, `usize exceeds repair time`, and `id recycling time`.

### Self Increasing
**10 Times.**

Total time consumption: 2585ns (2.58μs, 0ms).

Average time consumption: 139ns (0.14μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/normal_getting_time_10.png)

**100 Times.**

Total time consumption: 15509ns (15.51μs, 0.02ms).

Average time consumption: 105ns (0.10μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/normal_getting_time_100.png)

**1000 Times.**

Total time consumption: 153759ns (153.76μs, 0.15ms).

Average time consumption: 115ns (0.12μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/normal_getting_time_1000.png)

**10000 Times.**

Total time consumption: 1305586ns (1305.59μs, 1.31ms).

Average time consumption: 94ns (0.09μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/normal_getting_time_10000.png)

### USize Exceeds Repair Time
The average time spent in *200 tests* is: 98ns (0.10μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/usize_exceeding_correction_time_200.png)

### ID Recycling and Reuse Time
**Child Benchmark** Release ID Time.

Test at times 10, 100, 1000, 10000, 100000, 1000000.

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/id_recycling_and_reuse_time_release.png)

**Child Benchmark** Reuse ID Time.

Test at times 10, 100, 1000, 10000, 100000, 1000000.

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/id_recycling_and_reuse_time_reuse.png)


## InstantBus
This benchmark test will test the reception delay of different numbers of receivers (core).

### 1 Cores / Receivers
Average time consumption: 0ns (0μs, 0ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/instant_bus_1.png)
### 32 Cores / Receivers
Average time consumption: 48553ns (48.55μs, 0.05ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/instant_bus_32.png)
### 64 Cores / Receivers
Average time consumption: 106291ns (106.29μs, 0.11ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/instant_bus_64.png)
### 256 Cores / Receivers
Average time consumption: 485374ns (485.37μs, 0.49ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/instant_bus_256.png)
### 1024 Cores / Receivers
Average time consumption: 2168524ns (2168.52μs, 2.17ms).

![Benchmark](https://raw.githubusercontent.com/dimfunc/forever-safer/benchmarks/images/instant_bus_1024.png)