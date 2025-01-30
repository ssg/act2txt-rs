use std::{fs::File, io::Read, path::PathBuf, process::ExitCode};

use act::ReadError;
use clap::Parser;
use log::{error, warn};
use env_logger::{Builder, Env};

mod act;
mod txt;

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Params {
    #[arg(short = 'f', long = "force", help = "Overwrite <output> if it exists")]
    overwrite: bool,

    #[arg(
        long = "all",
        help = "Force convert all colors (even if extra data indicates otherwise)"
    )]
    all: bool,

    #[arg(help = "The input file to use (ACT format)")]
    input_filename: PathBuf,

    #[arg(help = "The output file to write (Paint.NET TXT format)")]
    output_filename: PathBuf,
}

macro_rules! failure {
    ($arg:expr) => {{
        error!("{}", $arg);
        ExitCode::FAILURE
    }};
    ($($arg:tt)+) => {{
        error!($($arg)+);
        ExitCode::FAILURE
    }};
}

fn main() -> ExitCode {
    Builder::from_env(Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .init();
    let args = Params::parse();

    let Ok(mut in_file) = File::open(&args.input_filename) else {
        return failure!("Could not open input file: {:?}", args.input_filename);
    };

    let palette = match act::Palette::read(&mut in_file, args.all) {
        Ok(p) => p,
        Err(e) => {
            return match e {
                ReadError::InvalidFileLength => {
                    failure!(
                        "File is too short, possibly an invalid ACT file: {}",
                        args.input_filename.display()
                    )
                }
                ReadError::IoError => {
                    failure!(
                        "Could not read input file: {}",
                        args.input_filename.display()
                    )
                }
            }
        }
    };

    // check if any bytes remaining after reading
    if in_file.read(&mut [0; 1]).is_ok() {
        warn!(
            "Input file is longer than expected, possibly an invalid ACT file: {}",
            args.input_filename.display()
        );
    }

    if let Some(index) = palette.transparent_index {
        if index > act::MAX_COLORS as u16 {
            warn!(
                "Transparent color index ({}) is too large, possibly an invalid ACT file: {}",
                index,
                args.input_filename.display()
            );
        }
    }

    let result = if args.overwrite {
        File::create(&args.output_filename)
    } else {
        File::create_new(&args.output_filename)
    };
    let Ok(mut out_file) = result else {
        return failure!("Could not create file: {:?}", args.output_filename);
    };

    if palette.write_pdn_txt(&mut out_file).is_err() {
        return failure!(
            "Could not write palette to output file {}",
            args.output_filename.display()
        );
    }

    ExitCode::SUCCESS
}
