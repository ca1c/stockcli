extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use anyhow::{Result};
use clap::Parser;
use ureq::{Error};


#[derive(Parser)]
struct Cli {
    symbol: String,
}

fn main() {
    dotenv().ok();

    let args = Cli::parse();
    let env_vars = init_env_vars();

    let stock_info = match get_stock_info(&env_vars, &args.symbol) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    println!("{}", stock_info);
}

fn init_env_vars() -> HashMap<String, String> {
    let mut env_vars = HashMap::new();
    
    for (var_key, env_var) in env::vars() {
        env_vars.insert(var_key.to_string(), env_var);
    }

    env_vars
}

fn get_stock_info(env_vars: &HashMap<String, String>, symbol: &String) -> Result<String, Error> {
    let token_value = env_vars.get("API_KEY").unwrap();

    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, token_value);

    let response: String = ureq::get(url.as_str())
        .call()?
        .into_string()?;

    Ok(response)
}

