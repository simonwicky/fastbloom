use crate::bloom::{BloomFilter, CountingBloomFilter};

/// Builder for Bloom Filters.
#[derive(Clone)]
#[derive(Debug)]
pub struct FilterBuilder {
    pub expected_elements: u64,
    pub false_positive_probability: f64,
    pub size: u64,
    pub hashes: u32,
    /// Usage for CountingBloomFilter.
    pub enable_repeat_insert: bool,
    pub(crate) done: bool,
}

#[cfg(target_pointer_width = "32")]
pub(crate) const SUFFIX: usize = 0b0001_1111;
#[cfg(target_pointer_width = "64")]
pub(crate) const SUFFIX: usize = 0b0011_1111;
#[cfg(target_pointer_width = "32")]
pub(crate) const MASK: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11100000;
#[cfg(target_pointer_width = "64")]
pub(crate) const MASK: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11000000;

/// Calculates the optimal size `m` of the bloom filter in bits given `n` (expected
/// number of elements in bloom filter) and `p` (tolerable false positive rate).
#[inline]
fn optimal_m(n: u64, p: f64) -> u64 {
    let fact = -(n as f64) * p.ln();
    let div = 2f64.ln().powi(2);
    let m: f64 = fact / div;
    let mut m = m.ceil() as u64;
    if (m & SUFFIX as u64) != 0 {
        m = (m & MASK) + SUFFIX as u64 + 1;
    };
    m
}

/// Calculates the optimal `hashes` (number of hash function) given `n` (expected number of
/// elements in bloom filter) and `m` (size of bloom filter in bits).
#[inline]
fn optimal_k(n: u64, m: u64) -> u32 {
    let k: f64 = (m as f64 * 2f64.ln()) / n as f64;
    k.ceil() as u32
}

/// Calculates the amount of elements a Bloom filter for which the given configuration of size `m`
/// and hashes `k` is optimal.
#[inline]
fn optimal_n(k: u32, m: u64) -> u64 {
    let n = (2f64.ln() * m as f64) / k as f64;
    n.ceil() as u64
}


/// Calculates the best-case (uniform hash function) false positive probability.
/// `k` number of hashes.
/// `m` The size of the bloom filter in bits.
/// `n` number of elements inserted in the filter.
#[inline]
fn optimal_p(k: u32, m: u64, n: u64) -> f64 {
    let nk = -(k as f64);
    (1.0 - (nk * n as f64 / m as f64).exp()).powi(k as i32)
}

impl FilterBuilder {
    /// Constructs a new Bloom Filter Builder by specifying the expected size of the filter and the
    /// tolerable false positive probability. The size of the BLoom filter in in bits and the
    /// optimal number of hash functions will be inferred from this.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastbloom_rs::FilterBuilder;
    /// let mut builder = FilterBuilder::new(100_000_000, 0.01);
    /// let bloom = builder.build_bloom_filter();
    ///
    /// ```
    pub fn new(expected_elements: u64, false_positive_probability: f64) -> Self {
        FilterBuilder {
            expected_elements,
            false_positive_probability,
            size: 0,
            hashes: 0,
            enable_repeat_insert: true,
            done: false,
        }
    }

    /// Constructs a new Bloom Filter Builder by specifying the size of the bloom filter in bits
    /// and the number of hashes. The expected size of the filter and the tolerable false positive
    /// probability will be inferred from this.
    pub fn from_size_and_hashes(size: u64, hashes: u32) -> Self {
        let n = optimal_n(hashes, size);
        let p = optimal_p(hashes, size, n);
        FilterBuilder {
            expected_elements: n,
            false_positive_probability: p,
            size,
            hashes,
            enable_repeat_insert: true,
            done: true,
        }
    }

    /// Use for CountingBloomFilter.
    ///
    /// # Example:
    /// ```rust
    /// use fastbloom_rs::{Deletable, FilterBuilder, Membership};
    ///
    /// let mut builder = FilterBuilder::new(100_000, 0.01);
    /// // enable_repeat_insert is true
    /// builder.enable_repeat_insert(true);
    /// let mut cbf = builder.build_counting_bloom_filter();
    /// cbf.add(b"hello"); // modify underlying vector counter.
    /// cbf.add(b"hello"); // modify underlying vector counter.
    /// assert_eq!(cbf.contains(b"hello"), true);
    /// cbf.remove(b"hello");
    /// assert_eq!(cbf.contains(b"hello"), true);
    /// cbf.remove(b"hello");
    /// assert_eq!(cbf.contains(b"hello"), false);
    ///
    /// // enable_repeat_insert is false
    /// builder.enable_repeat_insert(false);
    /// let mut cbf = builder.build_counting_bloom_filter();
    /// cbf.add(b"hello"); // modify underlying vector counter.
    /// cbf.add(b"hello"); // not modify underlying vector counter because b"hello" has been added.
    /// assert_eq!(cbf.contains(b"hello"), true);
    /// cbf.remove(b"hello");
    /// assert_eq!(cbf.contains(b"hello"), false);
    /// ```
    pub fn enable_repeat_insert(&mut self, enable: bool) {
        self.enable_repeat_insert = enable;
    }



    /// Checks if all necessary parameters were set and tries to infer optimal parameters (e.g.
    /// size and hashes from given expected_elements (`n`) and falsePositiveProbability (`p`)).
    /// This is done automatically.
    pub(crate) fn complete(&mut self) {
        if !self.done {
            if self.size == 0 {
                self.size = optimal_m(self.expected_elements, self.false_positive_probability);
                self.hashes = optimal_k(self.expected_elements, self.size);
            }
            self.done = true;
        }
    }

    /// Constructs a Bloom filter using the specified parameters and computing missing parameters
    /// if possible (e.g. the optimal Bloom filter bit size).
    pub fn build_bloom_filter(&mut self) -> BloomFilter {
        self.complete();
        BloomFilter::new(self.clone())
    }

    /// Constructs a Counting Bloom filter using the specified parameters and computing missing parameters
    /// if possible (e.g. the optimal Bloom filter bit size).
    pub fn build_counting_bloom_filter(&mut self) -> CountingBloomFilter {
        self.complete();
        CountingBloomFilter::new(self.clone())
    }

    /// Checks whether a configuration is compatible to another configuration based on the size of
    /// the Bloom filter and its hash functions.
    pub(crate) fn is_compatible_to(&self, other: &FilterBuilder) -> bool {
        self.size == other.size && self.hashes == other.hashes
    }
}


#[cfg(test)]
mod builder_tests {
    use super::*;
    use crate::Membership;

    #[test]
    fn optimal_test() {
        let m = optimal_m(100_000_000, 0.01);
        let k = optimal_k(100_000_000, m);
        let n = optimal_n(k, m);
        let p = optimal_p(k, m, n);
        println!("{m} {k} {n} {p}");
        assert_eq!(m, 958505856);
        assert_eq!(k, 7)
    }

    #[test]
    fn builder_test() {
        let mut bloom = FilterBuilder::new(100_000_000, 0.01)
            .build_bloom_filter();
        bloom.add(b"helloworld");
        assert_eq!(bloom.contains(b"helloworld"), true);
        assert_eq!(bloom.contains(b"helloworld!"), false);
    }
}