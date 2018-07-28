

use image;
use image::imageops;

pub fn hash_img(filename : &String) -> image::ImageResult<u64> {
    image::open(filename).map(|i| {
        let i = imageops::grayscale(&i.resize_exact(8,8,image::FilterType::Lanczos3));;
        let sum : u16 = i.pixels().map(|p| p[0] as u16).sum();
        let avg = sum / 64;

        let mut hash : u64 = 0;
        i.pixels().for_each(|p| {
            hash <<= 1;
            if p[0] as u16 >= avg {
                hash ^= 1;
            }
        });
        hash
    })
}