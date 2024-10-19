use clap::{Arg, Command};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::copy;

fn main() {
    let matches = Command::new("File Compressor CLI")
        .version("1.0")
        .author("Chukwuebuka <ebulamicheal@gmail.com>")
        .about("Compress files")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")  // This replaces .takes_value(true)
                .help("Input your file to compress")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")  // This replaces .takes_value(true)
                .help("The output compressed file"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").expect("Input file is required");
    let output_file = matches.get_one::<String>("output").map_or("output.gz", |s| s.as_str());

    compress_file(input_file, output_file).expect("Compression failed");
}

fn compress_file(input: &str, output: &str) -> std::io::Result<()> {
    let input_file = File::open(input)?;
    let output_file = File::create(output)?;

    let mut reader = BufReader::new(input_file);
    let writer = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(writer, Compression::default());
    copy(&mut reader, &mut encoder)?;
    encoder.finish()?;

    println!("The file has been compressed to {}", output);
    Ok(())
}
