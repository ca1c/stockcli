extern crate dotenv;
extern crate cfonts;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use anyhow::{Result};
use clap::Parser;
// use ureq::{Error};
use serde::{Deserialize, Serialize};
use serde_json::Result as SerdeResult;
use serde_json::Value;
use cfonts::{ say, Options, Fonts, Colors };
use colored::Colorize;


#[derive(Parser)]
struct Cli {
    #[clap(long, short, action)]
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

    if &stock_info == "null" {
        println!("cannot find this stock, so we did a search for it\n");

        let _ = search(&env_vars, &args.symbol);
    }
    else {
        match parse_response(&stock_info, &args.symbol) {
            Ok(stock_quote_struct) => {
                display_quote(&stock_quote_struct);
            }
            Err(err) => {
                eprintln!("Error parsing response: {}", err);
                return;
            }
        };
    }

    // display_quote(&stock_info);
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

    if response.contains("null") {
        Ok(String::from("null"))
    }
    else {
        Ok(response)
    }
}

fn parse_response(json_response: &String, symbol: &String) -> SerdeResult<StockQuote> {
    let mut copy_json_response = (*json_response).clone();
    copy_json_response.remove(0);

    let combined_json_response = format!("{{\"symbol\":\"{}\",{}", (*symbol).clone(), copy_json_response);

    let parsed_response: StockQuote = serde_json::from_str(combined_json_response.as_str())?;

    Ok(parsed_response)
}

fn display_quote(stock_quote: &StockQuote) {
    say(Options {
        text: stock_quote.symbol.clone(),
        font: Fonts::FontBlock,
        colors: vec![Colors::Green],
        ..Options::default()
    });

    let change_operator = String::from("+");
    let mut change_quote = format!("{}{} %{}", change_operator, stock_quote.d, f32::trunc(stock_quote.dp * 100.0) / 100.0).green();

    if stock_quote.d < 0.0 {
        change_quote = format!("{} %{}", stock_quote.d, f32::trunc(stock_quote.dp * 100.0) / 100.0).red();
    }

    println!("Market Price: ${}\n", stock_quote.c.to_string().blue());
    println!("Change: {}\n", change_quote);
    println!("High:${}  Low:${}\n", stock_quote.h.to_string().blue(), stock_quote.l.to_string().blue());
    println!("Open:${} Prev Close:${}", stock_quote.o.to_string().blue(), stock_quote.pc.to_string().blue());
}

// if the symbol provided returns no quote then a search result list will be displayed 
fn search(env_vars: &HashMap<String, String>, symbol: &String) -> Result<()> {
    let token_value = env_vars.get("API_KEY").unwrap();

    let url = format!("https://finnhub.io/api/v1/search?q={}&token={}", symbol, token_value);

    let response: String = ureq::get(url.as_str())
        .call()?
        .into_string()?;

    let search_json: Value = serde_json::from_str(response.as_str())?;

    let search_results: usize = search_json["count"].to_string().parse().unwrap();

    println!("Search results: {}", search_results);

    for i in 0..=search_results-1 {
        let curr_el = &search_json["result"][i];
        println!("Symbol: {} Description: {} Type: {}", curr_el["symbol"], curr_el["description"], curr_el["type"])
    }

    Ok(())
}

