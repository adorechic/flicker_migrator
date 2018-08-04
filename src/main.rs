extern crate reqwest;
extern crate rand;
extern crate time;
extern crate ring;
extern crate base64;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use reqwest::Url;
use rand::Rng;
use ring::{digest, hmac};

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
    let url = Url::parse_with_params(
        "https://api.flickr.com/services/rest/",
        &[
            ("method", "flickr.test.echo"),
            ("name", "value"),
            ("api_key", &api_key)
        ]
    ).unwrap();
    let body = reqwest::get(url).unwrap().text();

    println!("body = {:?}", body);
}

fn auth() {
    println!("auth!");
    let nonce = rand::thread_rng()
        .gen_ascii_chars()
        .take(32)
        .collect::<String>();
    let timestamp = format!("{}", time::now_utc().to_timespec().sec);
    let mut file = File::open("./api_key").expect("File not found");
    let mut api_key = String::new();
    file.read_to_string(&mut api_key).expect("Read error");
    let consumer_key = &api_key;
    let query = &[
        ("oauth_nonce", nonce),
        ("oauth_timestamp", timestamp),
        ("oauth_consumer_key", consumer_key.to_owned()),
        ("oauth_signature_method", "HMAC-SHA1".to_owned()),
        ("oauth_version", "1.0".to_owned()),
        ("oauth_callback", "http%3A%2F%2Flocalhost".to_owned())
    ];
    let base = query.iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .fold(
            String::from("GET&https%3A%2F%2Fwww.flickr.com%2Fservices%2Foauth%2Frequest_token?"),
            |acc, q| format!("{}&{}", acc, q)
        );
    let sign_key = format!("{}?", consumer_key);
    let signing_key = hmac::SigningKey::new(&digest::SHA1, sign_key.as_bytes());
    let signature = hmac::sign(&signing_key, base.as_bytes());
    let oauth_signature = base64::encode(signature.as_ref());
    let mut q = query.to_vec();
    q.push(("oauth_signature", oauth_signature));
    let url = q.iter().map(|(k, v)| format!("{}={}", k, v))
    .fold(
            String::from("https://www.flickr.com/services/oauth/request_token?"),
            |acc, q| format!("{}&{}", acc, q)
        );

    println!("url = {}", url);
    let body = reqwest::get(&url).unwrap().text();

    println!("body = {:?}", body);
}