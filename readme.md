# Rust Bloom Filter Implementation

This project provides a Rust implementation of a Bloom filter, inspired by LevelDB's Bloom filter. A Bloom filter is a
space-efficient probabilistic data structure used to test whether an element is a member of a set.

## Features

- Custom bits per key configuration
- Optimized hash function
- Efficient filter creation and querying

## Usage

### Creating a Bloom Filter

To create a new Bloom filter:

```rust
let bits_per_key = 10;
let bloom_filter = BloomFilter::new(bits_per_key);
```

The `bits_per_key` parameter determines the size and accuracy of the filter. A higher value will result in a larger
filter with a lower false positive rate.

### Creating a Filter from Keys

To create a filter from a set of keys:

```rust
let keys: Vec<Vec<u8> > = vec![
    b"apple".to_vec(),
    b"banana".to_vec(),
    b"cherry".to_vec(),
];
let filter = bloom_filter.create_filter( & keys);
```

### Checking for Key Presence

To check if a key may be present in the filter:

```rust
let key = b"apple";
let may_exist = bloom_filter.key_may_match(key, & filter);
```

Note that Bloom filters may return false positives but never false negatives. A `true` result means the key may be
present, while a `false` result means the key is definitely not present.

## Implementation Details

- The number of hash functions (`k`) is calculated based on the `bits_per_key` parameter, optimized for best
  performance.
- The filter uses a single hash function with a rotating delta to simulate multiple hash functions, improving
  performance.
- The filter size is adjusted to be a multiple of 8 bits for efficient storage.

## Performance Considerations

- The Bloom filter is designed to be space-efficient while maintaining a low false positive rate.
- The implementation uses bitwise operations for efficient querying.
- The filter size grows linearly with the number of keys, making it suitable for large datasets.

## Dependencies

This implementation relies on a custom hash function `bloom_hash` defined in the `hash` module.

## License

MIT

## Contributing

[Include information about how to contribute to the project]