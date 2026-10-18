
use std::hint::black_box;

use crate::r;
extern crate test;


#[bench]
fn bench_ranged(b: &mut test::Bencher) {
    b.iter(black_box(|| black_box(bench_fn_ranged())));
}


#[bench]
fn bench_int(b: &mut test::Bencher) {
    b.iter(black_box(|| black_box(bench_fn_int())));
}


fn bench_fn_ranged() -> u64 {
    let mut checksum: u64 = 0;

    let min_state = r!(0);
    let max_state = r!(1_000_000);
    let range_size = max_state - min_state + r!(1);

    for i in r!(1..=100) {
        let mut state = min_state + (i * r!(12345)) % range_size;
        for _ in 0..=99 {
            let current = state;

            let next_val = current * r!(1_103_515_245) + r!(12345);

            let bounded_val = min_state + (next_val % range_size);

            state = bounded_val;
            
            let mod_idx = r!(10) + (bounded_val % r!(490));
            let modif = state * mod_idx;
            
            checksum = checksum.wrapping_add(modif.u64());
        }
    }

    checksum
}


fn bench_fn_int() -> u64 {
    let mut checksum: u64 = 0;

    let min_state = 0;
    let max_state = 1_000_000;
    let range_size = max_state - min_state + 1;

    for i in 1..=100 {
        let mut state: u32 = min_state + (i * 12345) % range_size;
        for _ in r!(0..=99) {
            let current = state;

            let next_val = u64::from(current) * 1_103_515_245 + 12345;

            let bounded_val = min_state + unsafe{u32::try_from(next_val % u64::from(range_size)).unwrap_unchecked()};

            state = bounded_val;
            
            let mod_idx = 10 + (bounded_val % 490);
            let modif = state * mod_idx;
            
            checksum = checksum.wrapping_add(u64::from(modif));
        }
    }

    checksum
}
