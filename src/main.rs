use std::{fs::File, process::ExitCode};

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

fn main() -> ExitCode {
    let cmd_args = std::env::args().skip(1).collect::<Vec<String>>();
    let Ok(args) = Params::parse_args_default(&cmd_args) else {
        println!("{}", Params::usage());
        return ExitCode::FAILURE;
    };

    if args.help {
        println!("{}", Params::usage());
        return ExitCode::FAILURE;
    }

    let Ok(mut in_file) = File::open(&args.input) else {
        println!("Error: Could not open input file {}", args.input);
        return ExitCode::FAILURE;
    };

    let Ok(palette) = act::Palette::read(&mut in_file, args.all) else {
        println!("Error: Could not read palette from input file {}", args.input);
        return ExitCode::FAILURE;
    };

    let result = if args.overwrite { File::create(&args.output) } else { File::create_new(&args.output) };    
    let Ok(mut out_file) = result else {
        println!("Error: Could not create file {}", args.output);
        return ExitCode::FAILURE;
    };

    if palette.write_pdn_txt(&mut out_file).is_err() {
        println!("Error: Could not write palette to output file {}", args.output);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
