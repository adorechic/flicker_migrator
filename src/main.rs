extern crate reqwest;

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please subcommand fetch");
    }

    match args[1].as_ref() {
        "fetch" => fetch_photos(),
        "auth" => auth(),
        _         => panic!("Unknown command: {}, Use fetch", args[1]),
    }
}

fn fetch_photos() {
    let mut file = File::open("./api_key").expect("File not found");
    let mut api_key = String::new();
    file.read_to_string(&mut api_key).expect("Read error");
    let uri = format!("https://api.flickr.com/services/rest/?method=flickr.test.echo&name=value&api_key={}", api_key);
    let body = reqwest::get(&uri).unwrap().text();

    println!("body = {:?}", body);
}

fn auth() {
    println!("auth!")
}