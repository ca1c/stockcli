extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use anyhow::{Result};
use clap::Parser;


#[derive(Parser)]
struct Cli {
    symbol: String,
}

fn main() -> Result<()> {
    dotenv().ok();

    let args = Cli::parse();
    let env_vars = init_env_vars();

    Ok(())
}

fn init_env_vars() -> HashMap<String, String> {
    let mut env_vars = HashMap::new();
    
    for (var_key, env_var) in env::vars() {
        env_vars.insert(var_key.to_string(), env_var);
    }

    env_vars
}

