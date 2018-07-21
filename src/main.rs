extern crate reqwest;

fn main() {
    let body = reqwest::get("https://www.rust-lang.org").unwrap().text();

    println!("body = {:?}", body);
}
