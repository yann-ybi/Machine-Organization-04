use csc411_image::{RgbImage, Read, Write};
use crate::codec::output::un_bitpack::un_quan::av_dct::un_block::cv_rgb::cv_rgb::*;
use crate::codec::output::un_bitpack::un_quan::av_dct::av_dct::*;
use crate::codec::output::un_bitpack::un_quan::un_quan::*;
use crate::codec::output::un_bitpack::un_bitpack::*;
use crate::codec::output::output::*;

// this function calls cv, av_dct, pack, bitpack and output_rpeg functions to compress and file and print it out to standard output
pub fn compress(fname: Option<String>) {

    let mut image = RgbImage::read(fname.as_deref()).unwrap();

    let mut comp_arr2 = cv(&mut image);
    let mut av_dct = av_dct( &mut comp_arr2);
    let mut pack = pack(&mut av_dct);
    let mut bitpack = bitpack(&mut pack);
    output_rpeg (&mut bitpack)
}

// this function calls read_rpeg, un_bitpack, un_pack, inv_dct and rgb functions to decompress a file and print it out to standard output
pub fn decompress(fname: Option<String>) {

    let mut bitpack = read_rpeg(fname.as_deref());
    let mut unbitpack = un_bitpack(&mut bitpack);
    let mut unpack = un_pack(&mut unbitpack);
    let mut inv_dct = inv_dct(&mut unpack);
    let decomp_img = rgb(&mut inv_dct);

    decomp_img.write(None).unwrap();

}

pub fn compress_and_decompress (fname: Option<String>) {
    let mut image = RgbImage::read(fname.as_deref()).unwrap();

    let mut comp_arr2 = cv(&mut image);
    let mut av_dct = av_dct( &mut comp_arr2);
    let mut pack = pack(&mut av_dct);
    let mut bitpack = bitpack(&mut pack);

    let mut unbitpack = un_bitpack(&mut bitpack);
    let mut unpack = un_pack(&mut unbitpack);
    let mut inv_dct = inv_dct(&mut unpack);
    let decomp_img = rgb(&mut inv_dct);

    decomp_img.write(Some("cdecompressed.ppm")).unwrap();
}