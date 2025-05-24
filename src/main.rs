use std::io::{self, Read, Seek, SeekFrom, Write};

use anyhow::Ok;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Rust tail implmentation")]
struct Args {
    file_name: String,

    #[arg(short, long)]
    number: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut file: std::fs::File = std::fs::File::open(args.file_name)?;

    const BUFFER_SIZE: usize = 1024 * 8;
    let mut buffer = [0u8; BUFFER_SIZE];
    if file.metadata()?.len() > BUFFER_SIZE as u64 {
        file.seek(SeekFrom::End(-(BUFFER_SIZE as i64)))?;
    }

    let mut counter = 0;
    let target_lines = args.number;
    let mut output = io::stdout();
    'outer: while counter < target_lines {
        let readed = file.read(&mut buffer)?;
        if readed == 0 {
            break; // EOF            
        }

        let buffer = &buffer[..readed];
        let mut look_buffer = buffer;
        while let Some(index) = memchr::memrchr(b'\n', look_buffer) {
            counter += 1;
            look_buffer = &look_buffer[..index];
            if counter == target_lines {
                output.write_all(&buffer[index..])?;
                break 'outer;
            }
        }

        if file.seek_relative(-(BUFFER_SIZE as i64) * 2).is_err() {
            break; // EOF
        }
    }

    loop {
        let readed = file.read(&mut buffer)?;
        if readed == 0 {
            break; // EOF
        }
        let buffer = &buffer[..readed];
        output.write_all(buffer)?;
    }

    Ok(())
}
