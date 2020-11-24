use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Token {
    pub surf: String,
    pub base: String,
    pub category: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub token: Vec<Token>,
}

#[derive(Debug, Deserialize)]
pub struct ChildRule {
    pub child: String,
    pub rule: String,
}

#[derive(Debug, Deserialize)]
pub struct Terminal {
    pub terminal: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChildRuleTerminal {
    ChildRule { child: String, rule: String },
    Terminal { terminal: String },
}

#[derive(Debug, Deserialize)]
pub struct Span {
    #[serde(flatten)]
    pub childruleterminal: ChildRuleTerminal,
    pub category: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct CCG {
    pub score: String,
    pub id: String,
    pub root: String,
    pub span: Vec<Span>,
}

#[derive(Debug, Deserialize)]
pub struct Sentence {
    pub id: String,
    pub tokens: Tokens,
    pub ccg: Vec<CCG>,
}

#[derive(Debug, Deserialize)]
pub struct Sentences {
    pub sentence: Vec<Sentence>,
}
#[derive(Debug, Deserialize)]
pub struct Document {
    pub id: String,
    pub sentences: Sentences,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    pub document: Document,
}

pub fn parse(input: &str) -> Result<Root, serde_xml_rs::Error> {
    use serde_xml_rs::from_str;
    from_str(input)
}
