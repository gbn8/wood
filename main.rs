use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read}

pub fn main(){
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!({"string"})
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()>{
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf);

    run(&buf);
    Ok(())
}

fn run_prompt() {
    let stdin = io.stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line{
            if line.is_empty() {
                break;
            }
            run(&line.as_bytes());
        } else {
            break;
        }
    }
}

fn run(source: &[u8]) {
    let scanner = Scanner { source };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}