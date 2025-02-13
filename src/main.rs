use std::{fs::File, io::Read, path::PathBuf};
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

fn main() -> Result<(), anyhow::Error> {
    Builder::from_env(Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .init();
    let args = Params::parse();

    let mut in_file = File::open(&args.input_filename)
        .inspect_err(|_| error!("Could not open input file: {:?}", args.input_filename))?;

    let palette = act::Palette::read(&mut in_file, args.all)
        .inspect_err(|e| error!("{}: {}", e.to_string(), args.input_filename.display()))?;

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
    let mut out_file = result
        .inspect_err(|_| error!("Could not create output file: {:?}", args.output_filename))?;

    palette.write_pdn_txt(&mut out_file)
        .inspect_err(|_| error!("Could not write palette to output file: {:?}", args.output_filename))?;

    Ok(())
}
