#![no_main]
use libfuzzer_sys::fuzz_target;

extern crate bloomfilter;

fuzz_target!(|data: &[u8]| {
    let data_vec = data.to_vec();
    let mut bloom = bloomfilter::Bloom::new_for_fp_rate(128, 0.1);
    for i in 0..8 {
        for byte in data_vec.chunks((2 as usize).pow(i)) {
            bloom.set(byte);
        }

        for byte in data_vec.chunks((2 as usize).pow(i)) {
            assert!(bloom.check(byte));
        }
    }
});
