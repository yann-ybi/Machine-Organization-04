#![allow(unused)]

use std::{convert::TryInto, fmt::format};


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let range = 1 << width;
    if n >= -(range / 2) && n < (range / 2) {
        true
    } else{false}
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    let range = 1 << width;
    if  n < range {
        true
    } else{false}
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {

    if width > 64 || width + lsb > 64 {
        panic!("width and or lsb not in range (0, 64)");
    }
    let skip_by = 64 - (width + lsb) as usize;
    
    (((word as i64) << skip_by) >> (skip_by + lsb as usize)).try_into().unwrap()
    
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {

    if width > 64 || width + lsb > 64 {
        panic!("width and or lsb not in range (0, 64)");
    }
    let skip_by = 64 - (width + lsb) as usize;
    
    (word << skip_by) >> (skip_by + lsb as usize)
    
}

// this function returns a tuple ((a, b, c, d), pb, pr) from a word
pub fn get_all (word: u64) -> ((u64, i64, i64, i64), usize, usize) {
    ((getu(word, 9, 23), gets(word, 5, 18), gets(word, 5, 13), gets(word, 5, 8)), getu(word, 4, 4) as usize, getu(word, 4, 0) as usize)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if width > 64 || width + lsb > 64 {
        panic!("width and or lsb not in range (0, 64)");
    }
    let left_side = (word >> (lsb + width)) << (lsb + width);
    let right_side = (word << (32 - lsb)) >> (32 - lsb);
    let value = value << lsb;

    Some(left_side | right_side | value)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if width > 64 || width + lsb > 64 {
        panic!("width and or lsb not in range (0, 64)");
    }

    let left_side = (word >> (lsb + width)) << (lsb + width);
    let right_side = (word << (32 - lsb)) >> (32 - lsb);

    let mut value = ((value << (64 - width)) as u64) >> (64 - width);
    value = value << lsb;

    Some(left_side | right_side | value)
}

// this function takes a tuple ((a, b, c, d), pb, pr) and calls newu / news functions on each element
// returns a u32 word respecting the table on the csc 411 arith assignment requirements for compression
pub fn new_word(tuple: ((u64, i64, i64, i64), usize, usize)) -> u32{
    let mut word = newu(0, 4, 0, tuple.2 as u64).unwrap();
    word = newu(word, 4, 4, tuple.1 as u64).unwrap();
    word = news(word, 5, 8, (tuple.0).3).unwrap();
    word = news(word, 5, 13, (tuple.0).2).unwrap();
    word = news(word, 5, 18, (tuple.0).1).unwrap();
    word = newu(word, 9, 23, (tuple.0).0).unwrap();
    word as u32
}