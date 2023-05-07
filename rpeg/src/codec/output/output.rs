use array2::array2::Array2;
use csc411_rpegio::*;

// this function takes an array 2 of word u32, converts it to big endian bytes
// outputs the bytes to standard output
pub fn output_rpeg (arr2: &mut Array2<u32>){
    let output_vec =
    arr2.iter_row_major().map(|xyt| xyt.1)
        .map(|w| w.to_be_bytes())
        .collect::<Vec<[u8; 4]>>();
    // println!("Compressed image format 2\n{} {}", arr2.width_height.0, arr2.width_height.1);
    output_rpeg_data(&output_vec, arr2.width_height.0 as u32 * 2, arr2.width_height.1 as u32 * 2)
}

// this function reads a rpeg file, converts converts each big endian bytes word to u32
// returns an array 2 containing u32 words
pub fn read_rpeg (fname: Option<&str>) -> Array2<u32>{
 
    let read_tuple = read_in_rpeg_data(fname).unwrap();
    let vector = read_tuple.0;
    let dimension = (read_tuple.1 as usize / 2, read_tuple.2 as usize / 2);
    
    Array2::new(
        vector.into_iter().map(|be| u32::from_be_bytes(be))
    .collect::<Vec<u32>>(), dimension)

}