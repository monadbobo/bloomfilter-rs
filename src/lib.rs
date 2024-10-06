use crate::hash::bloom_hash;

mod hash;

pub struct BloomFilter {
    k: usize,
    bits_per_key: usize,
}

impl BloomFilter {
    pub fn new(bits_per_key: usize) -> BloomFilter {
        let mut k = (bits_per_key as f64 * 0.69) as usize;
        k = k.clamp(1, 30);

        BloomFilter { k, bits_per_key }
    }

    pub fn create_filter(&self, keys: &[Vec<u8>]) -> Vec<u8> {
        let mut bits = keys.len() * self.bits_per_key;

        if bits < 64 {
            bits = 64;
        }

        let bytes = (bits + 7) / 8;
        bits = bytes * 8;

        let mut filter = vec![0; bytes];
        filter.push(self.k as u8);

        for key in keys {
            let mut h = bloom_hash(key);
            let delta = h.rotate_right(17);

            for _ in 0..self.k {
                let bitpos = (h % bits as u32) as usize;
                filter[bitpos / 8] |= 1 << (bitpos % 8);
                h = h.wrapping_add(delta);
            }
        }
        filter
    }

    pub fn key_may_match(&self, key: &[u8], bloom_filter: &[u8]) -> bool {
        let len = bloom_filter.len();

        if len < 2 {
            return false;
        }

        let bits = (len - 1) * 8;

        let k = bloom_filter.last().unwrap();
        if *k > 30 {
            return true;
        }

        let mut h = bloom_hash(key);
        let delta = h.rotate_right(17);
        for _ in 0..*k {
            let bitpos = (h % bits as u32) as usize;
            if (bloom_filter[bitpos / 8] & (1 << (bitpos % 8))) == 0 {
                return false;
            }

            h = h.wrapping_add(delta);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;

    // Lower-level versions of Put... that write directly into a character buffer
    // REQUIRES: dst has enough space for the value being written
    fn encode_fixed32(value: u32) -> [u8; 4] {
        value.to_le_bytes()
    }

    #[test]
    fn test_empty_filter() {
        let filter = BloomFilter::new(10);
        assert!(!filter.key_may_match(b"hello", &[]));
        assert!(!filter.key_may_match(b"hello", &[]));
        assert!(!filter.key_may_match(b"x", &[]));
        assert!(!filter.key_may_match(b"foo", &[]));
    }

    #[test]
    fn test_small() {
        let bloom_filter = BloomFilter::new(10);
        let mut v = Vec::new();
        v.push(b"hello".to_vec());
        v.push(b"world".to_vec());
        let filter = bloom_filter.create_filter(v.as_slice());
        assert!(bloom_filter.key_may_match(b"hello", &filter));
        assert!(bloom_filter.key_may_match(b"world", &filter));
        assert!(!bloom_filter.key_may_match(b"x", &filter));
        assert!(!bloom_filter.key_may_match(b"foo", &filter));
    }

    fn next_length(length: usize) -> usize {
        if length < 10 {
            return length + 1;
        } else if length < 100 {
            return length + 10;
        } else if length < 1000 {
            return length + 100;
        }
        length + 1000
    }

    fn false_positive_rate(bloom_filter: &BloomFilter, filter: &[u8]) -> f64 {
        let mut result = 0;
        for i in 0..10000 {
            if bloom_filter.key_may_match(&encode_fixed32(i + 1000000000), filter) {
                result += 1;
            }
        }
        result as f64 / 10000.0
    }

    #[test]
    fn test_varing_length() {
        let bloom_filter = BloomFilter::new(10);
        let mut mediocre_filters = 0;
        let mut good_filters = 0;

        let mut length = 1;

        loop {
            if length > 10000 {
                break;
            }

            let mut keys = Vec::with_capacity(length);
            for i in 0..length {
                keys.push(encode_fixed32(i as u32).to_vec());
            }
            let filter = bloom_filter.create_filter(keys.as_slice());

            assert!(filter.len() <= ((length * 10 / 8) + 40));

            for i in 0..length {
                assert!(bloom_filter.key_may_match(&encode_fixed32(i as u32), &filter));
            }

            let rate = false_positive_rate(&bloom_filter, &filter);

            println!(
                "False positives: {:5.2}% @ length = {:6} ; bytes = {:6}",
                rate * 100.0,
                length,
                filter.len() as i32
            );

            assert!(rate <= 0.02);
            if rate > 0.0125 {
                mediocre_filters += 1;
            } else {
                good_filters += 1;
            }

            length = next_length(length);
        }

        println!(
            "Filters: {} good, {} mediocre",
            good_filters,
            mediocre_filters
        );

        assert!(mediocre_filters <= good_filters / 5);
    }
}
