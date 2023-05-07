use array2::array2::Array2;

// this function takes an array 2 of ((a, b ,c, d) pb, pr) as ((f32, f32, f32), usize, usize) and converts it to an array 2 of ((u64, i64, i64, i64), usize, usize)
pub fn pack(arr2: &mut Array2<((f32, f32, f32, f32), usize, usize)>) -> Array2<((u64, i64, i64, i64), usize, usize)> {
     
    let mut three_bit_tuple: Vec<((f32, f32, f32, f32), usize, usize)> = Vec::new();

    arr2.iter_row_major().map(|xyt| xyt.1)

    .for_each(|((a, mut b, mut c, mut d), pb, pr)|  { // I could use fits and fitsu here || check if it fits 3 bits
       
        // fits((b * 50.0).round(), 3) || false <=> negative -15 | true <=> positive 15 
        if b.abs() > 0.3 {
            if b.is_sign_positive() { b = 0.3 } else{ b = -0.3}
        } 
        // fits((c * 50.0).round(), 3)
        if c.abs() > 0.3 {
            if c.is_sign_positive() { c = 0.3 } else{ c = -0.3}
        } 
        // fits((d * 50.0).round(), 3)
        if d.abs() > 0.3 {
            if d.is_sign_positive() { d = 0.3 } else{ d = -0.3}
        } 

        three_bit_tuple.push(((a, b, c, d), pb, pr));
    });

    Array2::new(
    three_bit_tuple.into_iter()
    .map(|((a, b, c, d), pb, pr)| ((a as f64, b as f64, c as f64, d as f64), pb, pr))
    .map(|((a, b, c, d), pb, pr)| (((a * 511.0) as u64, (b * 50.0).round() as i64, (c * 50.0).round() as i64, (d * 50.0).round() as i64), pb, pr))
    .collect::<Vec<((u64, i64, i64, i64), usize, usize)>>(), arr2.width_height)

}

// this function takes an array 2 of ((a, b, c, d), pb, pr) as ((u64, i64, i64, i64), usize, usize) and converts it to an array 2 of ((f32, f32, f32, f32), usize, usize)
pub fn un_pack(arr2: &mut Array2<((u64, i64, i64, i64), usize, usize)>) -> Array2<((f32, f32, f32, f32), usize, usize)>{
    Array2::new(

    arr2.iter_row_major().map(|xyt| xyt.1)

    .map(|((a, b, c, d), pb, pr)| ((a as f64, b as f64, c as f64, d as f64), pb, pr))
    .map(|((a, b, c, d), pb, pr)| ((a / 511.0, b / 50.0, c / 50.0, d / 50.0), pb, pr))
    .map(|((a, b, c, d), pb, pr)|((a as f32, b as f32, c as f32, d as f32), pb, pr))
    .collect::<Vec<((f32, f32, f32, f32), usize, usize)>>(), arr2.width_height)

} // 0.0203