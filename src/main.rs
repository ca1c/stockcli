extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use anyhow::{Result};
use clap::Parser;
// use ureq::{Error};
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;


#[derive(Parser)]
struct Cli {
    symbol: String,
}

/*
* c: current price
* d: change
* dp: percent change
* h: high price of the day
* l: low price of the day
* o: open price of the day
* pc: previous close price
*/

#[derive(Serialize, Deserialize)]
struct StockQuote {
    symbol: String,
    c: f32,
    d: f32,
    dp: f32,
    h: f32,
    l: f32,
    o: f32,
    pc: f32,
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

fn get_stock_info(env_vars: &HashMap<String, String>, symbol: &String) -> Result<String, Box<dyn std::error::Error>> {
    let token_value = env_vars.get("API_KEY").unwrap();

    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, token_value);

    let response: String = ureq::get(url.as_str())
        .call()?
        .into_string()?;
    
    let quote = parse_response(&response, symbol)?;

    println!("quote: {}", quote.c);

    Ok(response)
}

fn parse_response(json_response: &String, symbol: &String) -> SerdeResult<StockQuote> {
    let mut copy_json_response = (*json_response).clone();
    copy_json_response.remove(0);

    let combined_json_response = format!("{{\"symbol\":\"{}\",{}", (*symbol).clone(), copy_json_response);

    println!("{}", combined_json_response);

    let parsed_response: StockQuote = serde_json::from_str(combined_json_response.as_str())?;
    println!("{}", parsed_response.c);

    Ok(parsed_response)
}

