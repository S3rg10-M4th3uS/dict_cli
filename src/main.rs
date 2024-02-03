use crate::parsing::Response;
use anyhow::Result;
use clap::Parser;
use reqwest::blocking::get;

mod parsing;

#[derive(Parser)]
struct Args {
    name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        args.name
    );

    let body = get(url)?.text()?;
    let responses: Response = serde_json::from_str(&body)?;

    println!("{:?}", responses);
    Ok(())
}
