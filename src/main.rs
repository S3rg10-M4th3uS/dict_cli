use crate::parsing::Response;
use anyhow::Result;
use clap::Parser;
use parsing::Body;
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
    let responses = &responses[0];

    parse_response(responses);

    Ok(())
}

fn parse_response(body: &Body) {
    println!("{:<8} -- {}", "Word", body.word);
    if body.phonetic.is_some() {
        println!("{:<8} -- {}", "Phonetic", body.phonetic.as_ref().unwrap());
    }
    for meaning in body.meanings.iter() {
        for definitions in meaning.definitions.iter() {
            println!("{} /", definitions.definition);
        }
    }
}
