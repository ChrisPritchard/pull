use std::{env, process::exit, fs, io::{stdout, Write}};

use reqwest::blocking::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("args: url [outputfilename]");
        exit(1);
    }
    let client = Client::new();
    let bytes = client.get(&args[1]).send().and_then(|r| r.bytes()).expect("failed to call url");
    if args.len() > 2 {
        let path = &args[2];
        fs::write(path, &bytes).expect("failed to write file");
    } else {
        stdout().write(&bytes).expect("failed to write to stdout");
    }
}
