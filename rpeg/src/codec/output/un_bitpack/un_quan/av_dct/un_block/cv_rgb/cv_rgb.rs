use csc411_image::{self, RgbImage, Rgb};
use array2::array2::Array2;

// Takes an image
// return a trimmed even width and height
pub fn make_even (image: &mut RgbImage) -> (usize, usize) {
    let mut w_h = (image.width as usize, image.height as usize);
    if let 0 = w_h.0 % 2 {} else {w_h.0 -= 1}
    if let 0 = w_h.1 % 2 {} else {w_h.1 -= 1}
    return w_h
}

// takes an image calls, make_even on it, converts rgb (r, g, b) into component videos (y, pb, pr) of an array 2
pub fn cv(image: &mut RgbImage) -> Array2<(f32, f32, f32)> { // might be f64
    let w_h = make_even(image);
    
    let mut img_arr2 = Array2::new(image.pixels.clone(),w_h);
    let d = image.denominator as f64;
    let comp_video = img_arr2.iter_row_major()
    .map(|xyt| xyt.1)
    .map(|rgb| (rgb.red as f64 / d, rgb.green as f64 / d, rgb.blue as f64 / d))
    .map(|(r,g,b)| (0.299 * r + 0.587 * g + 0.114 * b, -0.168736 * r - 0.331264 * g + 0.5 * b, 0.5 * r - 0.418688 * g - 0.081312 * b))
    .map(|(y, pb, pr)| (y as f32, pb as f32, pr as f32))
    .collect::<Vec<(f32,f32,f32)>>();

    Array2::new(comp_video, w_h)
}

// takes an array2 component videos (y, pb, pr) and converts it into an array 2 of rgb (r, g, b)
pub fn rgb(arr2: &mut Array2<(f32, f32, f32)>) -> RgbImage{
    let w_h = arr2.width_height;
    let d = 255 as f64;
    let rgb = arr2.iter_row_major()
    .map(|xyt|xyt.1)
    .map(|cv| (cv.0 as f64, cv.1 as f64, cv.2 as f64))
    .map(|(y,pb,pr)| (1.0 * y + 0.0 * pb + 1.402 * pr, 1.0 * y - 0.344136 * pb - 0.714136 * pr, 1.0 * y + 1.772 * pb - 0.0 * pr))
    .map(|(r, g, b)| (r * d, g * d, b * d))
    .map(|(r, g, b)| Rgb{ red: r as u16, green: g as u16, blue: b as u16})
    .collect::<Vec<Rgb>>();

    RgbImage {
        pixels: rgb,
        width: w_h.0 as u32,
        height: w_h.1 as u32,
        denominator: d as u16,
    }
}