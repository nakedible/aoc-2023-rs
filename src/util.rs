use chumsky::prelude::*;

pub fn integer() -> impl Parser<char, i64, Error = Simple<char>> {
    text::int(10).try_map(|n: String, span| n.parse::<i64>().map_err(|e| Simple::custom(span, format!("{}", e))))
}
