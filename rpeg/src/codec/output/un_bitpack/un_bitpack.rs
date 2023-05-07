use array2::array2::Array2;
use bitpack::bitpack::*;

// this function takes an array 2 of tuples ((a, b, c, d), pb pr) and turns them into a u32 word using bitpack functions
// return array2 of words u32
pub fn bitpack (arr2: &mut Array2<((u64, i64, i64, i64), usize, usize)>) -> Array2<u32> {
        Array2::new(
        arr2.iter_row_major().map(|xyt| xyt.1)
        .map(|t| new_word(t))
        .collect::<Vec<u32>>(), arr2.width_height
        )
}

// this function takes an array 2 of u32 words, get a tuple ((a, b, c, d), pb, pr) from it
// returns an array 2 of these tuples
pub fn un_bitpack(arr2: &mut Array2<u32>) -> Array2<((u64, i64, i64, i64), usize, usize)> {
    Array2::new(
    arr2.iter_row_major().map(|xyt| xyt.1 as u64)
    .map(|w| get_all(w))
    .collect::<Vec<((u64, i64, i64, i64), usize, usize)>>(), arr2.width_height)
}