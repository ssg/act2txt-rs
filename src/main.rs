use std::{fs::File, process::ExitCode};

use act::ReadError;
use gumdrop::Options;

mod act;
mod txt;

#[derive(Options, Debug)]
struct Params {
    #[options(help = "Print this help message")]
    help: bool,

    #[options(short = "f", no_long, help = "Overwrite <output> if it exists")]
    overwrite: bool,

    #[options(no_short, long = "all", help = "Force convert all colors (even if extra data indicates otherwise)")]
    all: bool,

    #[options(free, required, help = "The input file to use (ACT format)")]
    input: String,

    #[options(free, required, help = "The output file to write (TXT format)")]
    output: String,
}

fn err(msg: String) -> ExitCode {
    println!("{}", msg);
    ExitCode::FAILURE
}

fn usage() -> ExitCode {
    err(Params::usage().to_string())
}

fn main() -> ExitCode {
    let cmd_args = std::env::args().skip(1).collect::<Vec<String>>();
    let Ok(args) = Params::parse_args_default(&cmd_args) else {
        return usage();
    };

    if args.help {
        return usage();
    }

    let Ok(mut in_file) = File::open(&args.input) else {
        return err(format!("Could not open input file: {}", args.input));
    };

    let palette = match act::Palette::read(&mut in_file, args.all) {
        Ok(p) => p,
        Err(e) => return match e {
            ReadError::InvalidFileLength => err(format!("Invalid input file length: {}", args.input)),
            ReadError::IoError => err(format!("Could not read input file: {}", args.input)),
        }    
    };

    let result = if args.overwrite { File::create(&args.output) } else { File::create_new(&args.output) };    
    let Ok(mut out_file) = result else {
        return err(format!("Could not create file: {}", args.output));
    };

    if palette.write_pdn_txt(&mut out_file).is_err() {
        return err(format!("Error: Could not write palette to output file {}", args.output));
    }

    ExitCode::SUCCESS
}
