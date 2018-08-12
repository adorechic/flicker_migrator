extern crate reqwest;
extern crate time;
extern crate base64;
extern crate crypto;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use reqwest::Url;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

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
    let api_key = read_file("./api_key".to_string());
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

fn read_file(path: String) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Read error");
    return s;
}

fn auth() {
    println!("auth!");
    let nonce = String::from("89601180");
    let timestamp = format!("{}", time::now_utc().to_timespec().sec);
    let consumer_key = read_file("./api_key".to_string());
    let consumer_secret = read_file("./consumer_secret".to_string());
    let query = &[
        ("oauth_callback", "http%253A%252F%252Flocalhost".to_owned()),
        ("oauth_consumer_key", consumer_key.to_owned()),
        ("oauth_nonce", nonce.to_string()),
        ("oauth_signature_method", "HMAC-SHA1".to_owned()),
        ("oauth_timestamp", timestamp.to_string()),
        ("oauth_version", "1.0".to_owned())
    ];
    let query_string = query.iter().map(|(k, v)| format!("{}%3D{}", k, v)).collect::<Vec<_>>().join("%26");
    let base = format!("{}&{}", 
        "GET&https%3A%2F%2Fwww.flickr.com%2Fservices%2Foauth%2Frequest_token",
        query_string
    );

    let key = format!("{}&", consumer_secret);
    let mut hmac = Hmac::new(Sha1::new(), key.as_bytes());
    hmac.input(base.as_bytes());
    let oauth_signature = base64::encode(hmac.result().code());
    let q = &[
        ("oauth_callback", "http%3A%2F%2Flocalhost".to_owned()),
        ("oauth_consumer_key", consumer_key.to_owned()),
        ("oauth_nonce", nonce),
        ("oauth_signature_method", "HMAC-SHA1".to_owned()),
        ("oauth_timestamp", timestamp),
        ("oauth_version", "1.0".to_owned())
    ];
    let q_string = q.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
    let url_str = format!("{}?{}", "https://www.flickr.com/services/oauth/request_token", q_string);
   
    let mut url = Url::parse(&url_str).unwrap();
    url.query_pairs_mut().append_pair("oauth_signature", &oauth_signature);
    let body = reqwest::get(url).unwrap().text();
    println!("body = {:?}", body);
}