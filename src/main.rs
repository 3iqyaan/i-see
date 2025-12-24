mod cli;

use clap::Parser;
use cli::Isee;

use std::fs;
use std::io::{BufReader, Read};
use std::time::Instant;

fn main() -> std::io::Result<()>{

    let start = Instant::now();

    let args = Isee::parse();

    let file_names = args.file;
    let byte = match args.byte{
        Some(b) => b,
        None => b'\n'
    };
    for file_name in file_names{
        let file_name = file_name.trim();
        let c = count(&file_name.replace("\"", ""), byte)?;
        println!("There were {} lines in file: {}", c, file_name);
        println!("Execution took time: {}ms", start.elapsed().as_millis());
    }


    Ok(())
}

fn count(file_name: &str, byte: u8) -> std::io::Result<u64>{

    let mut buf = [0u8; 65536];
    let mut c: u64 = 0;
    let f = fs::File::open(file_name)?;
    let mut f = BufReader::with_capacity(65536, f);

    loop {
        let n = f.read(&mut buf)?;
        for &b in &buf[..n]{
            if byte == b{
                c += 1;
            }
        }
        if n == 0 {
            break;
        }
    }
    Ok(c)
}