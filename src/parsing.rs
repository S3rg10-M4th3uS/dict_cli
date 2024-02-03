use serde::Deserialize;

pub type Response = Vec<Body>;

#[derive(Debug, Deserialize, Default)]
pub struct Body {
    pub word: String,
    pub phonetic: Option<String>,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Meaning {
    pub part_of_speech: Option<String>,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
}
