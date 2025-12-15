use anyhow::{Context, Ok, Result};
use dotenv::dotenv;
use reqwest::Client;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env::{self},
    process::{self, Command},
    time::Duration,
};
use tokio::time::interval;

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
    let server_ip = get_ip()?;

    let mut body = HashMap::new();
    body.insert("secretapikey", &secretapikey);
    body.insert("apikey", &apikey);
    body.insert("content", &server_ip);

    let args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        eprintln!("Too many arguments");
        process::exit(0)
    }
    let site = args.get(1).context("Need site name argument")?;
    let duration = args
        .get(2)
        .context("Need time duration argument")?
        .parse()
        .context("Not a valid integer")?;

    let url = "https://api.porkbun.com/api/json/v3/dns/";
    let client = reqwest::Client::new();
    let mut interval = interval(Duration::from_mins(duration));
    loop {
        interval.tick().await;
        println!("Checking IP change for {site}...");
        edit_a_records(url, &body, &client, &site).await?;
        println!("Checking records again in {duration} minutes...");
        println!("----------");
        ()
    }
}

fn get_ip() -> Result<String> {
    let ip = Command::new("sh")
        .arg("-c")
        .arg("curl ifconfig.me")
        .output()?;

    Ok(String::from_utf8(ip.stdout)?.trim().to_string())
}

async fn get_list_of_subdomains(
    url: &str,
    body: &HashMap<&str, &String>,
    client: &Client,
    site: &String,
) -> Result<Vec<String>> {
    let mut sub_vector: Vec<String> = Vec::new();
    let api_url = format!("{url}retrieve/{site}");

    let post_request = client.post(api_url).json(&body).send().await?;
    let result: GetRecords = post_request
        .json()
        .await
        .context("Did you enable API access for this domain?")?;

    for entry in result.records {
        if entry.r_type == "A" {
            let sub: Vec<&str> = entry.name.split(".").collect();
            if sub.len() > 2 {
                sub_vector.push(String::from(sub[0]));
            }
        }
    }
    Ok(sub_vector)
}

async fn edit_a_records(
    url: &str,
    body: &HashMap<&str, &String>,
    client: &Client,
    site: &String,
) -> Result<()> {
    let api_url = format!("{url}editByNameType/{site}/A/");
    let subdomains = get_list_of_subdomains(url, &body, &client, &site).await?;
    let server_ip = &body.get("content").or(None).context("Error parsing IP")?;
    let post_request = client.post(&api_url).json(&body).send().await?;

    if post_request.error_for_status().is_err() {
        println!("IP has not changed!")
    } else {
        println!("IP has changed to {server_ip}! Updating IPs!");
        println!("----------");
        println!("Root record updated!");
        println!("----------");
        for sub in subdomains {
            let api_url = format!("{api_url}{sub}");
            client.post(api_url).json(&body).send().await?;
            println!("Subdomain {sub} record updated!");
            println!("----------")
        }
    };
    Ok(())
}
