<h1>fastbloom</h1>

A fast [bloom filter](#BloomFilter) | [counting bloom filter](#countingbloomfilter) implemented by Rust for Rust

Language: [简体中文](./docs/README.zh_cn.md)

- [setup](#setup)
    - [Rust](#rust)
- [Examples](#examples)
    - [BloomFilter](#bloomfilter)
        - [Rust](#rust-1)
    - [CountingBloomFilter](#countingbloomfilter)
        - [Rust](#rust-2)
- [benchmark](#benchmark)
    - [computer info](#computer-info)
    - [add one str to bloom filter](#add-one-str-to-bloom-filter)
    - [add one million to bloom filter](#add-one-million-to-bloom-filter)
    - [check one contains in bloom filter](#check-one-contains-in-bloom-filter)
    - [check one not contains in bloom filter](#check-one-not-contains-in-bloom-filter)
    - [add one str to counting bloom filter](#add-one-str-to-counting-bloom-filter)
    - [add one million to counting bloom filter](#add-one-million-to-counting-bloom-filter)

# setup

## Rust

```toml
fastbloom-rs = "{latest}"
```

# Examples

## BloomFilter

A Bloom filter is a space-efficient probabilistic data structure, conceived by Burton Howard
Bloom in 1970, that is used to test whether an element is a member of a set. False positive
matches are possible, but false negatives are not.

**Reference**: Bloom, B. H. (1970). Space/time trade-offs in hash coding with allowable errors.
Communications of the ACM, 13(7), 422-426.
[Full text article](http://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf)

### Rust

```rust
use fastbloom_rs::{BloomFilter, FilterBuilder};

let mut bloom = FilterBuilder::new(100_000_000, 0.01).build_bloom_filter();

bloom.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
assert_eq!(bloom.contains(b"helloworld!"), false);
```

more examples at [docs.rs](https://docs.rs/fastbloom-rs)

## CountingBloomFilter

A Counting Bloom filter works in a similar manner as a regular Bloom filter; however, it is
able to keep track of insertions and deletions. In a counting Bloom filter, each entry in the
Bloom filter is a small counter associated with a basic Bloom filter bit.

**Reference**: F. Bonomi, M. Mitzenmacher, R. Panigrahy, S. Singh, and G. Varghese, “An Improved
Construction for Counting Bloom Filters,” in 14th Annual European Symposium on
Algorithms, LNCS 4168, 2006

### Rust

```rust
use fastbloom_rs::{CountingBloomFilter, FilterBuilder};

let mut builder = FilterBuilder::new(100_000, 0.01);
let mut cbf = builder.build_counting_bloom_filter();
cbf.add(b"helloworld");
assert_eq!(bloom.contains(b"helloworld"), true);
```

# benchmark

## computer info

| CPU                                    | Memory | OS         |
|----------------------------------------|--------|------------|
| AMD Ryzen 7 5800U with Radeon Graphics | 16G    | Windows 10 |

## add one str to bloom filter

Benchmark insert one str to bloom filter:

```text
bloom_add_test          time:   [41.168 ns 41.199 ns 41.233 ns]
                        change: [-0.4891% -0.0259% +0.3417%] (p = 0.91 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
```

## add one million to bloom filter

Benchmark loop insert `(1..1_000_000).map(|n| { n.to_string() })` to bloom filter:

```text
bloom_add_all_test      time:   [236.24 ms 236.86 ms 237.55 ms]
                        change: [-3.4346% -2.9050% -2.3524%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

## check one contains in bloom filter

```text
bloom_contains_test     time:   [42.065 ns 42.102 ns 42.156 ns]
                        change: [-0.7830% -0.5901% -0.4029%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  9 (9.00%) high severe
```

## check one not contains in bloom filter

```text
bloom_not_contains_test time:   [22.695 ns 22.727 ns 22.773 ns]
                        change: [-3.1948% -2.9695% -2.7268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
```

## add one str to counting bloom filter

```text
counting_bloom_add_test time:   [60.822 ns 60.861 ns 60.912 ns]
                        change: [+0.2427% +0.3772% +0.5579%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe
```

## add one million to counting bloom filter

Benchmark loop insert `(1..1_000_000).map(|n| { n.to_string() })` to counting bloom filter:

```text
counting_bloom_add_million_test
                        time:   [272.48 ms 272.58 ms 272.68 ms]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
```
