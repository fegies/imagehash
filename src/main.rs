extern crate image;
extern crate rayon;

use rayon::prelude::*;

mod imghash;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

#[derive(Debug)]
enum Inputfile {
    Stdin,
    Filename(String),
}

enum InputfileIterator {
    StdinBase(Lines<BufReader<io::Stdin>>),
    FileBase(Lines<BufReader<File>>),
}

impl Iterator for InputfileIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            InputfileIterator::StdinBase(b) => b.next().map(|s| s.unwrap()),
            InputfileIterator::FileBase(b) => b.next().map(|s| s.unwrap()),
        }
    }
}

impl Inputfile {
    fn new(name: String) -> Inputfile {
        match name.as_ref() {
            "-" => Inputfile::Stdin,
            _ => Inputfile::Filename(name),
        }
    }
    fn lines(&self) -> InputfileIterator {
        match self {
            Inputfile::Stdin => InputfileIterator::StdinBase(BufReader::new(io::stdin()).lines()),
            Inputfile::Filename(f) => {
                InputfileIterator::FileBase(BufReader::new(File::open(f).unwrap()).lines())
            }
        }
    }
}

fn main() {
    let mut batchmode = false;

    let mut raw_files = Vec::new();
    let mut batch_files = Vec::new();
    env::args().skip(1).for_each(|arg| {
        match arg.as_ref() {
            "-b" => {
                batchmode = true;
            }
            _ if batchmode => {
                batch_files.push(Inputfile::new(arg));
            }
            _ => {
                raw_files.push(arg);
            }
        };
    });

    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    let all_files = raw_files
        .into_iter()
        .chain(batch_files.into_iter().flat_map(|f| f.lines()));

    all_files.par_bridge().for_each(|f| {
        let hash = imghash::hash_img(&f, 64);
        match hash {
            Ok(h) => {
                println!("{}\t->\t{}", f, h);
            }
            Err(e) => {
                println!("{}\t->\t{}", f, e);
            }
        }
    });
    // all_files.par_iter().for_each(|f| {
    //     println!("f");
    // });
    // .chunks(1000)
    // .into_iter()
    // .map(|a| a.collect::<Vec<_>>())
    // .for_each(|a| {
    //     a.par_iter()
    //         .map(|f| (imghash::hash_img(&f), f))
    //         .for_each(|(h, f)| {
    //             match h {
    //                 Ok(h) => println!("{}\t{}", h.to_string(), f),
    //                 Err(e) => eprintln!("Error: {} in File {}", e, f),
    //             };
    //         })
    // });
    // .for_each(|f| {
    //     pool.spawn(move || {
    //         let h = imghash::hash_img(&f);
    //         match h {
    //             Ok(h) => println!("{:016x}\t{}", h, f),
    //             Err(e) => eprintln!("Error: {} in File {}", e, f)
    //         };
    //     });
    // });
}
