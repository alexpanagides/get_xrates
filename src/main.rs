// use std::alloc::System;
use serde::Deserialize;
use reqwest::Error;
use serde_json::Value;
use tokio;

#[allow(dead_code)]

#[derive(Deserialize)]
struct ExResponse {
    success: bool
}

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
        let text = response.text().await?;
        // TODO get json
        // let v: Value = serde_json::from_str(&*text)?;
        // println!("SUCCESS: {}",v["success"]);

        // let users: Vec<User> = response.json().await?;
        // TODO convert to json and look for success == true
        //  if not true then don't overwrite existing file
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