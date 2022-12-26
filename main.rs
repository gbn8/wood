use std::io:{Read, File, BufReader};
use std::fs::File;
use std::env::args;

pub fn main(){
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        println!({"string"})
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(args[0]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String){
    let f = File::open(path).expect("Cannot open file");
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf);
}