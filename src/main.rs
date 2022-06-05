// use std::alloc::System;
// use serde::Deserialize;
use reqwest::Error;
use tokio;

// #[derive(Deserialize, Debug)]
#[allow(dead_code)]
// struct User {
//     login: String,
//     id: u32,
// }
#[tokio::main]
async fn main() -> Result<(), Error> {
    // let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    //                           owner = "rust-lang-nursery",
    //                           repo = "rust-cookbook");
    let request_url = String::from("https://api.exchangerate.host/latest?base=USD&symbols=USD,EUR,JPY");
    // println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    // if false {
    if response.status().is_success() {
        // let text = String::new();
        let text = response.text().await?;
        // let users: Vec<User> = response.json().await?;
        println!("{}", text);

        std::fs::write(
            "./exchange.json",
            &text,
        )
            .unwrap();

        return Ok(());
    }

    eprintln!("failure to get");
    std::process::exit(1)
}