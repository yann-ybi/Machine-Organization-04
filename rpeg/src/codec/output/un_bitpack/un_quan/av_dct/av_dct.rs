use array2::array2::Array2;
use crate::codec::output::un_bitpack::un_quan::av_dct::un_block::un_block::{make_blocks, undo_blocks};
use csc411_arith::*;

// takes an array2  of (y, pb, pr) and returns an array 2 of ((a, b, c, d), pb average, pr average) using the formulas from the assginment
// uses csc411_arith: index_of_chroma function to turn a f32 into a usize for pb and pr averages (quantize)
pub fn av_dct (arr2: &mut Array2<(f32, f32, f32)>) -> Array2<((f32, f32, f32, f32), usize, usize)>{

    let mut blocked = make_blocks(arr2);

    let iter0 = blocked.iter_row_major().map(|xyt|xyt.1)
    .map(|v| (v[0].0 as f64, v[1].0 as f64, v[2].0 as f64, v[3].0 as f64))
    .map(|(y1, y2, y3, y4)| ((y4 + y3 + y2 + y1), (y4 + y3 - y2 - y1), (y4 - y3 + y2 - y1), (y4 - y3 - y2 + y1)))
    .map(|(a4, b4, c4, d4)| (a4 / 4.0, b4 / 4.0, c4 / 4.0, d4 / 4.0))
    .map(|(a, b, c, d)| (a as f32, b as f32, c as f32, d as f32));

    
    let iter1 = blocked.iter_row_major().map(|xyt| xyt.1)
    .map(|v| ((v[0].1 as f64, v[1].1 as f64, v[2].1 as f64, v[3].1 as f64), (v[0].2 as f64, v[1].2 as f64, v[2].2 as f64, v[3].2 as f64)))
    .map(|(pbs, prs)| (pbs.0 + pbs.1 + pbs.2 + pbs.3, prs.0 + prs.1 + prs.2 + prs.3))
    .map(|(pb4, pr4)|  ((pb4 / 4.0) as f32, (pr4 / 4.0) as f32))
    .map(|f| (index_of_chroma(f.0 as f32), index_of_chroma(f.1 as f32)));
    
    
    let av_dct_vec = iter0.zip(iter1)
    .map(|z| (((z.0).0, (z.0).1, (z.0).2, (z.0).3), (z.1).0, (z.1).1))
    .collect::<Vec<((f32, f32, f32, f32), usize, usize)>>();

    Array2::new(av_dct_vec, blocked.width_height ) 

}

// takes an array 2 of ((a, b, c, d), pb average, pr average) amd return an array 2 of tuple (y, pb average, pr average)
// uses csc411_arith: chroma_of_index function to turn a usize into a f32 for pb and pr averages (unquantize)
pub fn inv_dct (arr2: &mut Array2<((f32, f32, f32, f32), usize, usize)>) -> Array2<(f32, f32, f32)> {

    let mut inv_dct_vec: Vec<Vec<(f32, f32, f32)>> = Vec::new();
    
    let iter0 = arr2.iter_row_major().map(|xyt| xyt.1)

    .map(|t| ((t.0).0 as f64, (t.0).1 as f64, (t.0).2 as f64, (t.0).3 as f64))
    .map(|(a, b, c, d)| (a - b - c + d, a - b + c - d, a + b - c - d, a + b + c + d))
    .map(|(y1, y2, y3, y4)| (y1 as f32, y2 as f32, y3 as f32, y4 as f32) );
    
    let iter1 = arr2.iter_row_major().map(|xyt| xyt.1)
    .map(|t| (chroma_of_index(t.1) , chroma_of_index(t.2)));

    iter0.zip(iter1)
    .map(|r| (((r.0).0, (r.1).0, (r.1).1), ((r.0).1, (r.1).0, (r.1).1), ((r.0).2, (r.1).0, (r.1).1), ((r.0).3, (r.1).0, (r.1).1) ) )
    .for_each( |(y1s, y2s, y3s, y4s)| {
        inv_dct_vec.push(
            vec![y1s, y2s, y3s, y4s]
        )
    } );

    undo_blocks(&mut Array2::new(inv_dct_vec, arr2.width_height))

}