extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::process::exit;
use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        exit(1);
    }

    let input_file = args().nth(1).unwrap();
    let output_file = args().nth(2).unwrap();

    let mut input = BufReader::new(File::open(input_file).unwrap());
    let output = File::create(output_file).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::best());
    let start = Instant::now();

    copy(&mut input, &mut encoder)?;
    let output = encoder.finish()?;

    println!("Source len: {:?}", input.get_ref().metadata()?.len());

    println!("Target len: {:?}", output.metadata()?.len());
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
