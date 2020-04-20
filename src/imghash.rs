use image::{
    imageops::{self, FilterType::Triangle},
    ImageError,
};
use std::fmt::{self, Display};

pub struct Hash {
    backing: Vec<u64>,
    size: usize,
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nibbles = self.size as i64 / 4;
        let mut index = 0;
        while nibbles > 0 {
            let width = usize::min(nibbles as usize, 16);
            f.write_fmt(format_args!(
                "{:0width$x}",
                self.backing[index],
                width = width,
            ))?;
            nibbles -= 16;
            index += 1;
        }
        Ok(())
    }
}

struct HashBuilder {
    backing: Vec<u64>,
    target_size: usize,
    size: usize,
}

pub enum HashError {
    NotEnoughBits,
    DecoderError(ImageError),
}

impl From<ImageError> for HashError {
    fn from(from: ImageError) -> Self {
        HashError::DecoderError(from)
    }
}

impl Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HashError::NotEnoughBits => f.write_fmt(format_args!("Not enough bits in the buffer")),
            HashError::DecoderError(inner) => f.write_fmt(format_args!("Decoder Error: {}", inner)),
        }
    }
}

impl HashBuilder {
    fn new(size: usize) -> HashBuilder {
        let full_words = size / 64;
        let extra_word_needed = size % 64 > 0;
        let word_count = full_words + if extra_word_needed { 1 } else { 0 };
        HashBuilder {
            backing: Vec::with_capacity(word_count),
            target_size: size,
            size: 0,
        }
    }
    fn add_bit(&mut self, is_one: bool) {
        if self.size % 64 == 0 {
            self.backing.push(if is_one { 1 } else { 0 });
        } else {
            let hash_part = self
                .backing
                .last_mut()
                .expect("This cannot happen because the size would be zero otherwise");
            *hash_part <<= 1;
            if is_one {
                *hash_part ^= 1;
            }
        }
        self.size += 1;
    }

    fn finalize(self) -> Result<Hash, HashError> {
        if self.size < self.target_size {
            return Err(HashError::NotEnoughBits);
        }
        Ok(Hash {
            backing: self.backing,
            size: self.target_size,
        })
    }
}

pub fn hash_img(filename: &str, hash_width: u32, hash_height: u32) -> Result<Hash, HashError> {
    let img = image::open(filename)?;
    let img = imageops::grayscale(&img.resize_exact(hash_width, hash_height, Triangle));

    let hash_size: usize = (hash_height * hash_width) as usize;
    let sum: u64 = img.pixels().map(|p| p[0] as u64).sum();
    let avg = sum / (hash_size as u64);

    let mut hash = HashBuilder::new(hash_size);
    for p in img.pixels() {
        hash.add_bit(p[0] as u64 >= avg);
    }
    hash.finalize()
}
