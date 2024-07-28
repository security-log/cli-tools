use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of 'cat'
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,

    /// Number lines
    #[arg(conflicts_with("number_nonblank_lines"), short('n'), long("number"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(conflicts_with("number_lines"), short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,

    /// Show ends
    #[arg(short('E'), long("show-ends"))]
    show_ends: bool,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprint!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if args.number_lines {
                        print!("{:>6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            print!("");
                        } else {
                            prev_num += 1;
                            print!("{prev_num:>6}\t{line}");
                        }
                    } else {
                        print!("{line}");
                    }

                    if args.show_ends {
                        println!("$");
                    } else {
                        println!();
                    }
                }
            }
        }
    }
    Ok(())
}

// Box guarda el retorno del dyn al heap, esto es necesario ya
// que Rust no sabe la cant de memoria a usar en el stack.
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
