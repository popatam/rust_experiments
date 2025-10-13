use std::hash::{Hash, Hasher};

pub struct BloomFilter {
    bitmap: Vec<u8>,
    bitmap_bits_size: usize,
    hash_funcs: Vec<fn(&[u8]) -> u64>,
}

pub struct MyHasher {
    hash: u64,
}

impl Default for MyHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl MyHasher {
    pub fn new() -> Self {
        MyHasher { hash: 0 }
    }

    pub fn new_with_seed(seed: u64) -> Self {
        MyHasher { hash: seed }
    }
}

impl Hasher for MyHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64;
            self.hash = self.hash.wrapping_mul(self.hash)
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

fn hash1(bytes: &[u8]) -> u64 {
    let mut hasher = MyHasher::new_with_seed(1);
    bytes.hash(&mut hasher);
    hasher.finish()
}

fn hash2(bytes: &[u8]) -> u64 {
    let mut hasher = MyHasher::new_with_seed(20);
    bytes.hash(&mut hasher);
    hasher.finish()
}

impl BloomFilter {
    pub fn new(size: u64, hash_funcs: Vec<fn(&[u8]) -> u64>) -> Self {
        // size - размер в битах
        let bytes_count = size.div_ceil(8) as usize; // округление вверх
        let bitmap = vec![0u8; bytes_count];
        Self {
            bitmap,
            bitmap_bits_size: size as usize,
            hash_funcs,
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.bitmap
    }

    pub fn info(&self) {
        println!("Bloom filter size: {}", self.bitmap.len());
        for b in &self.bitmap {
            print!("{:08b}.", b);
        }
        println!("\n");
        for b in 0..self.bitmap_bits_size {
            let byte_index = b / 8;
            let bit_in_byte = b % 8;
            let is_set = self.bitmap[byte_index] & (1u8 << bit_in_byte) != 0;
            print!("{}", if is_set { "1" } else { "0" });
            if (b + 1) % 8 == 0 {
                print!(".");
            }
        }
        println!("\nBloom filter: {:?}", self.bitmap);
    }

    pub fn insert(&mut self, data: &[u8]) {
        for hash_func in &self.hash_funcs {
            let bit = hash_func(data) as usize;
            let bit_index_in_bitmap_bound = bit % self.bitmap_bits_size;

            let byte_index = bit_index_in_bitmap_bound / 8;
            let bit_index_in_byte = bit_index_in_bitmap_bound % 8;

            self.bitmap[byte_index] |= 1 << bit_index_in_byte; // установкa бита bit_index_in_byte в 1 в байте byte_index
        }
    }
    pub fn maybe_contain(&self, data: &[u8]) -> bool {
        for hash_func in &self.hash_funcs {
            let bit = hash_func(data) as usize;
            let bit_index_in_bitmap_bound = bit % self.bitmap_bits_size;

            let byte_index = bit_index_in_bitmap_bound / 8;
            let bit_index_in_byte = bit_index_in_bitmap_bound % 8;

            if ((self.bitmap[byte_index]) & (1 << bit_index_in_byte)) == 0 {
                return false;
            };
        }
        true
    }
}

fn main() {
    let hash_funcs = vec![hash1, hash2];
    let mut bf = BloomFilter::new(64, hash_funcs);
    bf.info();
    bf.insert("hellp".as_bytes());
    bf.info();
    bf.insert("world".as_bytes());
    bf.info();
    for &w in ["a", "b", "c", "d", "e", "f"].iter() {
        println!(
            "word: {} is in filter: {}",
            w,
            bf.maybe_contain(w.as_bytes())
        );
    }
}
