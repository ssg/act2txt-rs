use std::{fs::File, process::ExitCode};

use act::ReadError;
use clap::Parser;

mod act;
mod txt;

#[derive(Parser, Debug)]
struct Params {
    #[arg(short = 'f', long = "force", help = "Overwrite <output> if it exists")]
    overwrite: bool,

    #[arg(long = "all", help = "Force convert all colors (even if extra data indicates otherwise)")]
    all: bool,

    #[arg(help = "The input file to use (ACT format)")]
    input: String,

    #[arg(help = "The output file to write (Paint.NET TXT format)")]
    output: String,
}

macro_rules! failure {
    ($arg:expr) => {{
        eprintln!("{}", $arg);
        ExitCode::FAILURE
    }};
    ($($arg:tt)+) => {{
        eprintln!($($arg)+);
        ExitCode::FAILURE
    }};
}

fn main() -> ExitCode {
    let args = Params::parse();

    let Ok(mut in_file) = File::open(&args.input) else {
        return failure!("Could not open input file: {}", args.input);
    };

    let palette = match act::Palette::read(&mut in_file, args.all) {
        Ok(p) => p,
        Err(e) => return match e {
            ReadError::InvalidFileLength => failure!("Invalid input file length: {}", args.input),
            ReadError::IoError => failure!("Could not read input file: {}", args.input),
        }    
    };

    let result = if args.overwrite { File::create(&args.output) } else { File::create_new(&args.output) };    
    let Ok(mut out_file) = result else {
        return failure!("Could not create file: {}", args.output);
    };

    if palette.write_pdn_txt(&mut out_file).is_err() {
        return failure!("Could not write palette to output file {}", args.output);
    }

    ExitCode::SUCCESS
}
