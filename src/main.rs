use anyhow::{Ok, Result};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::{
    env::{self},
    process::{self, Command},
    time::Duration,
};

#[derive(Deserialize, Debug)]
struct GetRecords {
    records: Vec<Record>,
}

#[derive(Deserialize, Debug)]
struct Record {
    name: String,
    #[serde(rename = "type")]
    r_type: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let secretapikey = env::var("secretapikey")?;
    let apikey = env::var("apikey")?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        eprintln!("Too many arguments");
        process::exit(0)
    }
    let site = args.get(1).expect("Site argument missing...");
    let duration = args
        .get(2)
        .expect("Duration argument missing...")
        .parse()
        .expect("Not a valid integer");

    let url = "https://api.porkbun.com/api/json/v3/dns/";
    let client = reqwest::Client::new();

    loop {
        println!("Checking IP change for {site}...");
        edit_a_records(url, &secretapikey, &apikey, &client, &site).await?;
        println!("Checking records again in {duration} minutes...");
        println!("----------");
        tokio::time::sleep(Duration::from_mins(duration)).await;
    }
}

fn get_ip() -> String {
    let ip = Command::new("sh")
        .arg("-c")
        .arg("curl ifconfig.me")
        .output()
        .expect("Error curling request...");
    String::from_utf8(ip.stdout)
        .expect("Could not convert to string...")
        .trim()
        .to_string()
}

async fn get_list_of_subdomains(
    url: &str,
    secretapikey: &String,
    apikey: &String,
    client: &Client,
    site: &String,
) -> Vec<String> {
    let body = serde_json::json!({
        "secretapikey" : secretapikey,
        "apikey" : apikey
    });

    let mut sub_vector: Vec<String> = Vec::new();
    let api_url = format!("{url}retrieve/{site}");

    let post_request = client
        .post(api_url)
        .json(&body)
        .send()
        .await
        .expect("Unable to post request...");

    let result: GetRecords = post_request
        .json()
        .await
        .expect("Unable to obtain records... Is the API enabled in porkbun for the domain?");

    for entry in result.records {
        if entry.r_type == 'A'.to_string() {
            let sub: Vec<&str> = entry.name.split(".").collect();
            if sub.len() > 2 {
                sub_vector.push(String::from(sub[0]));
            }
        }
    }
    sub_vector
}

async fn edit_a_records(
    url: &str,
    secretapikey: &String,
    apikey: &String,
    client: &Client,
    site: &String,
) -> Result<()> {
    let server_ip = get_ip();
    let api_url = format!("{url}editByNameType/{site}/A/");

    let subdomains = get_list_of_subdomains(url, &secretapikey, &apikey, &client, &site).await;

    let body = serde_json::json!({
        "secretapikey" : secretapikey,
        "apikey" : apikey,
        "content": server_ip
    });

    let post_request = client.post(&api_url).json(&body).send().await?;

    if post_request.error_for_status().is_err() {
        println!("IP has not changed!")
    } else {
        println!("IP has changed! Updating IPs!");
        println!("----------");
        println!("Root record updated!");
        println!("----------");
        for sub in subdomains {
            let api_url = format!("{api_url}{sub}");
            client.post(api_url).json(&body).send().await?;
            println!("Subdomain {sub} record updated!");
            println!("----------")
        }
        println!("All domains updated to reflect {server_ip}!")
    };
    Ok(())
}
