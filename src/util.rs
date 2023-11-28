use chumsky::prelude::*;

pub fn errstr(errs: Vec<Simple<char>>) -> anyhow::Error {
    anyhow::anyhow!(errs.into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n"))
}

pub fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10).try_map(|n: String, span| n.parse::<i64>().map_err(|e| Simple::custom(span, format!("{}", e))))
}
