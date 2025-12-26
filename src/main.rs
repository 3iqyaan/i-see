mod cli;

use clap::Parser;
use cli::Isee;

use std::fs;
use std::io::{BufReader, Read};
use std::time::Instant;

fn main() -> std::io::Result<()>{

    let start = Instant::now();

    let args = Isee::parse();

    match args.target{
        cli::Target::File(fcmd) => {
            analyze_file(fcmd)?;
        }
        cli::Target::Dir(dcmd) => {
            analyze_dir(dcmd)?;
        }
    }
    println!("Execution took time: {}ms", start.elapsed().as_millis());

    Ok(())
}

fn countb(file_name: &str, byte: u8) -> std::io::Result<u64>{

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

fn analyze_file(args: cli::FCmd) -> std::io::Result<()> {
    let file_names: Vec<String> = args.file;
    let v = args.verbose;

    for file_name in file_names{
        let file_name = file_name.trim();
        let file_name = file_name.replace("\"", "");
        if let Some(b) = args.byte{
            let cb = countb(&file_name, b)?;
            if v{
                println!("There were {} occurrences of byte {} in file: {}", cb, b, file_name);
            }
        }
        if args.line{
            let cl = countb(&file_name, b'\n')?;
            if v{
                println!("There were {} lines in file: {}", cl, file_name);
            }
        }
    }
    Ok(())
}

fn analyze_dir(args: cli::DCmd) -> std::io::Result<()> {
    let v = args.verbose;
    let depth = match args.recursive {
        true => None,
        false => Some(1),
    };

    let walker = ignore::WalkBuilder::new(&args.path)
                .hidden(!args.hidden)
                .git_ignore(!args.no_gitignore)
                .max_depth(depth)
                .build();

    for path in walker{
        let path = path.unwrap().path().to_path_buf();
        if path.is_file(){
            if let Some(ext) = &args.extension{
                if let Some(e) = path.extension(){
                    if e.to_str().unwrap() != ext{
                        continue;
                    }
                }
                else{
                    continue;
                }
            }
            if let Some(b) = args.byte{
                let c = countb(&path.to_string_lossy().to_string(), b)?;
                if v {
                    println!("There were {} occurrences of byte {} in file: {}", c, b, path.display());
                }
            }
            if args.line{
                let c = countb(&path.to_string_lossy().to_string(), b'\n')?;
                if v {
                    println!("There were {} lines in file: {}", c, path.display());
                }
            }
        }
        else if args.recursive && path.is_dir(){
            
        }
    }
    Ok(())
}