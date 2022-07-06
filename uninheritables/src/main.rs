use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::io::Write;
use rand::Rng;

fn main() {
    match run() {
        Ok(_) => { println!("Done!") }
        Err(error) => { println!("Error: {}", error) }
    }
}

struct ValueGenerator<R: Rng> {
    rng: R,
}

impl<R: Rng> ValueGenerator<R> {
    fn new(rng: R) -> ValueGenerator<R> {
        ValueGenerator { rng }
    }
    fn zero_or_one(&mut self, p: f64) -> u8 {
        if self.rng.gen_bool(p) { 1 } else { 0 }
    }
}

struct Args {
    in_file: String,
    out_file: String,
}

fn read_args() -> Result<Args, Box<dyn Error>> {
    let mut in_file: Option<String> = None;
    let mut out_file: Option<String> = None;
    let mut next_is_in_file = false;
    let mut next_is_out_file = false;
    for arg in env::args() {
        if next_is_in_file {
            in_file = Some(arg);
            next_is_in_file = false
        } else if next_is_out_file {
            out_file = Some(arg);
            next_is_out_file = false;
        } else if arg == "-i" {
            next_is_in_file = true
        } else if arg == "-o" {
            next_is_out_file = true;
        }
    }
    match (in_file, out_file) {
        (Some(in_file), Some(out_file)) => { Ok(Args { in_file, out_file }) }
        (None, _) => { Err("Missing input file")? }
        (_, None) => { Err("Missing output file")? }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = read_args()?;
    println!("in: {}", args.in_file);
    let reader = BufReader::new(File::open(args.in_file)?);
    println!("out: {}", args.out_file);
    let mut writer = BufWriter::new(File::create(args.out_file)?);
    writeln!(&mut writer, "id,spirit0,spirit1,spirit2,covariate0,covariate1,covariate2")?;
    for line in reader.lines() {
        let id = line?;
        let mut gen = ValueGenerator::new(rand::thread_rng());
        let spirit0 = gen.zero_or_one(0.1);
        let spirit1 = gen.zero_or_one(0.2);
        let spirit2 = gen.zero_or_one(0.3);
        let covariate0 = gen.zero_or_one(0.4);
        let covariate1 = gen.zero_or_one(0.5);
        let covariate2 = gen.zero_or_one(0.6);
        writeln!(&mut writer, "{},{},{},{},{},{},{}", id, spirit0, spirit1, spirit2, covariate0,
                 covariate1, covariate2)?;
    }
    Ok(())
}