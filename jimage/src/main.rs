use std::env::args;

use jimage::image_file::ImageFile;

fn main() {
    let mut args = args().skip(1);
    let path = args.next().unwrap();
    let image = ImageFile::open(&path).unwrap();
    println!("{:#02X?}", image.header);
    println!("{:?}", image.index);
}
