mod codec;
use crate::codec::codec::{compress,decompress, compress_and_decompress};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let fname = args.clone().into_iter().nth(2);
    match args[1].as_str() {
        "-c" => compress(fname),
        "-d" => decompress(fname),
        "-cd" => compress_and_decompress(fname),
        _ => {
        eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }

}