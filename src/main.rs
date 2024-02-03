use anyhow::Result;
use clap::Parser;
use reqwest::blocking::get;

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
    println!("{:?}", body);
    Ok(())
}
