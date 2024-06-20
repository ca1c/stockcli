extern crate dotenv;
extern crate cfonts;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use anyhow::{Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cfonts::{ say, Options, Fonts, Colors };
use colored::Colorize;


#[derive(Parser)]
struct Cli {
    #[clap(long, short, action)]
    symbol: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Cli::parse();
    let env_vars: HashMap<String, String> = init_env_vars()?;

    let stock_info = get_stock_info(&env_vars, &args.symbol)?;

    if &stock_info == "null" {
        println!("cannot find this stock, so we did a search for it\n");

        search(&env_vars, &args.symbol)?;
    }
    else {
        let mut stock_quote: StockQuote = StockQuote::new();
        stock_quote.parse_response(&stock_info, &args.symbol)?;
        stock_quote.display_quote();
    }

    Ok(())
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

impl StockQuote {
    pub fn new() -> Self {
        StockQuote {
            symbol: String::from("XXXXX"),
            c: 0.0,
            d: 0.0,
            dp: 0.0,
            h: 0.0,
            l: 0.0,
            o: 0.0,
            pc: 0.0,
        }
    }

    fn display_quote(&self) {
        say(Options {
            text: self.symbol.clone(),
            font: Fonts::FontBlock,
            colors: vec![Colors::Green],
            ..Options::default()
        });
    
        let change_operator = String::from("+");
        let mut change_quote = format!("{}{} %{}", change_operator, self.d, f32::trunc(self.dp * 100.0) / 100.0).green();
    
        if self.d < 0.0 {
            change_quote = format!("{} %{}", self.d, f32::trunc(self.dp * 100.0) / 100.0).red();
        }
    
        println!("Market Price: ${}\n", self.c.to_string().blue());
        println!("Change: {}\n", change_quote);
        println!("High:${}  Low:${}\n", self.h.to_string().blue(), self.l.to_string().blue());
        println!("Open:${} Prev Close:${}", self.o.to_string().blue(), self.pc.to_string().blue());
    }

    fn parse_response(&mut self, json_response: &String, symbol: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut copy_json_response = (*json_response).clone();
        copy_json_response.remove(0);
    
        let combined_json_response = format!("{{\"symbol\":\"{}\",{}", (*symbol).clone(), copy_json_response);
    
        let parsed_response: StockQuote = serde_json::from_str(combined_json_response.as_str())?;

        *self = parsed_response;

        Ok(())
    }
}

fn init_env_vars() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut env_vars = HashMap::new();
    
    for (var_key, env_var) in env::vars() {
        env_vars.insert(var_key.to_string(), env_var);
    }

    if !env_vars.contains_key("API_KEY") {
        return Err("API_KEY was not found. Please add API_KEY environment variable in a .env file in CLI root".into());
    }

    Ok(env_vars)
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

