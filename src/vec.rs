use crate::builder::SUFFIX;

/// bitmap only for bloom filter.
#[derive(Debug)]
#[derive(Clone)]
pub(crate) struct BloomBitVec {
    /// Internal representation of the bit vector
    pub(crate) storage: Vec<usize>,
}

impl BloomBitVec {
    pub fn new(slots: usize) -> Self {
        BloomBitVec {
            storage: vec![0; slots],
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 6;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 5;
        let b = index & SUFFIX;
        let flag = 1usize << b;
        self.storage[w] = self.storage[w] | flag;
    }

    #[inline]
    pub fn get(&self, index: usize) -> bool {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 6;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 5;
        let b = index & SUFFIX;
        let flag = 1usize << b;
        (self.storage[w] & flag) != 0
    }

    #[inline]
    pub fn get_and_set(&mut self, index: usize) -> bool {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 6;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 5;
        let b = index & SUFFIX;
        let flag = 1usize << b;
        let value = (self.storage[w] & flag) != 0;
        self.storage[w] = self.storage[w] | flag;
        value
    }

    pub fn or(&mut self, other: &BloomBitVec) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m |= *o;
        }
    }

    pub fn and(&mut self, other: &BloomBitVec) {
        for (m, o) in self.storage.iter_mut().zip(&other.storage) {
            *m &= *o;
        }
    }

    pub fn clear(&mut self) {
        self.storage.fill(0);
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }
}

/// counter vector for counting bloom filter.
#[derive(Debug)]
#[derive(Clone)]
pub(crate) struct CountingVec {
    /// Internal representation of the vector
    pub(crate) storage: Vec<usize>,

}

impl CountingVec {
    /// create a CountingVec
    pub fn new(slots: usize) -> Self {
        CountingVec {
            storage: vec![0; slots],
        }
    }

    #[inline]
    pub fn increment(&mut self, index: usize) {
        let current = self.get(index);
        #[cfg(target_pointer_width = "64")]
        if current != 0b1111 {
            let current = current + 1;
            let w = index >> 4;
            let b = index & 0b1111;
            let move_bits = (15 - b) * 4;
            self.storage[w] =
                (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
        }

        #[cfg(target_pointer_width = "32")]
        if current != 0b111 {
            let current = current + 1;
            let w = index >> 3;
            let b = index & 0b111;
            let move_bits = (7 - b) * 4;
            self.storage[w] =
                (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
        }
    }

    #[inline]
    pub fn decrement(&mut self, index: usize) {
        let current = self.get(index);
        if current > 0 {
            if cfg!(target_pointer_width="64") {
                let current = current - 1;
                let w = index >> 4;
                let b = index & 0b1111;
                let move_bits = (15 - b) * 4;
                self.storage[w] =
                    (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
            } else if cfg!(target_pointer_width="32") {
                let current = current - 1;
                let w = index >> 3;
                let b = index & 0b111;
                let move_bits = (7 - b) * 4;
                self.storage[w] =
                    (self.storage[w] & !(0b1111 << move_bits)) | (current << move_bits)
            }
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> usize {
        #[cfg(target_pointer_width = "64")]
            let w = index >> 4;
        #[cfg(target_pointer_width = "64")]
            let b = index & 0b1111;
        #[cfg(target_pointer_width = "32")]
            let w = index >> 3;
        #[cfg(target_pointer_width = "32")]
            let b = index & 0b111;
        let slot = self.storage[w];
        #[cfg(target_pointer_width = "64")]
        return (slot >> ((15 - b) * 4)) & 0b1111;
        #[cfg(target_pointer_width = "32")]
        return (slot >> ((7 - b) * 4)) & 0b111;
    }

    pub fn clear(&mut self) {
        self.storage.fill(0);
    }
}

#[cfg(test)]
mod vec_test {
    use super::*;

    #[test]
    fn test_vec() {
        let mut vec = BloomBitVec::new(16);
        vec.set(37);
        vec.set(38);
        println!("{:?}", vec);
        assert_eq!(vec.get(37), true);
        assert_eq!(vec.get(38), true);
    }


    #[test]
    fn test_count_vec() {
        let mut vec = CountingVec::new(10);
        vec.increment(7);

        assert_eq!(1, vec.get(7))
    }
}