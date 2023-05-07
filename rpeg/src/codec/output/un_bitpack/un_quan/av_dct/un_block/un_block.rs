use array2::array2::Array2;

// takes an array 2 of (y, pb, pr) returns an array 2 of vectors of (y, pb, pr)
// each vector, a 2 by 2 block of elements
pub fn make_blocks(arr2: &mut Array2<(f32, f32, f32)>) -> Array2<Vec<(f32, f32, f32)>>{
    let mut block = Array2::blanck_state(0, (2,2));
    let mut coords_block1_vec = block.iter_row_major().map(|xyt| xyt.0).collect::<Vec<(usize, usize)>>();
    let mut ini_block = coords_block1_vec.clone();

    let mut blocked_arr2_vec = Vec::new();

    for _i in 0..( arr2.width_height.0 * arr2.width_height.1 / 4) {

        blocked_arr2_vec.push(coords_block1_vec.iter()
        .map(|x| arr2.get_mut(*x).copied().unwrap()).collect::<Vec<(f32, f32, f32)>>());

        // function to go to the next block
        for coords in coords_block1_vec.iter_mut() {
            coords.0 = (coords.0 + 2) % arr2.width_height.0;
        }
        if coords_block1_vec == ini_block {
            for coords in coords_block1_vec.iter_mut() {coords.1 = (coords.1 + 2) % arr2.width_height.1};
            ini_block = coords_block1_vec.clone();
        }
        
    }
    Array2::new(blocked_arr2_vec, (arr2.width_height.0 / 2, arr2.width_height.1 / 2))

}

// takes an array 2 of vectors of tuples and returns an array 2 of tuples, putting each tuple at its respective position out of its 2 by 2 block
pub fn undo_blocks(blocked_arr2: &mut Array2<Vec<(f32, f32, f32)>>) -> Array2<(f32, f32, f32)>{

    let mut arr2_vec: Vec<(f32, f32, f32)> = Vec::new();
    let mut arr2_vec1: Vec<(f32, f32, f32)> =  Vec::new();
    let mut arr2_vec2: Vec<(f32, f32, f32)> = Vec::new();

    blocked_arr2.iter_row_major()
    .map(|xyt|xyt.1)
    .for_each(|f| {
        arr2_vec1.push(f[0]); arr2_vec1.push(f[1]);
        arr2_vec2.push(f[2]); arr2_vec2.push(f[3]);
    });
    let original_width = blocked_arr2.width_height.0 * 2;

    arr2_vec1.chunks_mut(original_width).zip(arr2_vec2.chunks_mut(original_width))
    .for_each (|(chunk1, chunk2)| {
        chunk1.iter().copied().chain(chunk2.iter().copied())
        .for_each(|t| arr2_vec.push(t));
    } );
    Array2::new(arr2_vec, (original_width, blocked_arr2.width_height.1 * 2)) 
}