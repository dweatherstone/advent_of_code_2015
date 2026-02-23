use std::sync::atomic::{AtomicU64, Ordering};

use openssl::hash::{Hasher, MessageDigest};
use rayon::prelude::*;

pub fn result_day04_stage1(input: &str) -> u64 {
    // 5 leading zeroes = 2 full bytes + upper nibble zero
    mine(input, 2, true)
}

pub fn result_day04_stage2(input: &str) -> u64 {
    // 6 leading zeroes = 3 full zero bytes
    mine(input, 3, false)
}

/// Shared mining function.
/// `zero_bytes` = number of leading zero bytes required
/// `extra_nibble` =  if true, require upper 4 bits of next byte to be zero (for 5 hex zeroes).
fn mine(input: &str, zero_bytes: usize, extra_nibble: bool) -> u64 {
    let found = AtomicU64::new(u64::MAX);

    // Precompute prefix state once
    let mut base = Hasher::new(MessageDigest::md5()).unwrap();
    base.update(input.as_bytes()).unwrap();

    // Chunk size balances scheduling overhead vs responsiveness
    const CHUNK_SIZE: u64 = 50_000;

    let mut start = 0u64;

    loop {
        let end = start + CHUNK_SIZE;

        (start..end).into_par_iter().for_each(|i| {
            // Early exit if another thread already found a smaller solution
            if i >= found.load(Ordering::Relaxed) {
                return;
            }

            let mut hasher = base.clone();
            hasher.update(i.to_string().as_bytes()).unwrap();

            let digest = hasher.finish().unwrap();

            // Check required zero bytes
            if digest[..zero_bytes].iter().any(|&b| b != 0) {
                return;
            }

            // Check optional half-byte (for 5 hex zeroes)
            if extra_nibble && digest[zero_bytes] >= 0x10 {
                return;
            }

            // Update minimum safely
            let mut current = found.load(Ordering::Relaxed);
            while i < current {
                match found.compare_exchange(current, i, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(v) => current = v,
                }
            }
        });

        let candidate = found.load(Ordering::Relaxed);
        if candidate != u64::MAX {
            return candidate;
        }

        start = end;
    }
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn stage1() {
        assert_eq!(result_day04_stage1("abcdef"), 609043);
        assert_eq!(result_day04_stage1("pqrstuv"), 1048970);
    }
}
