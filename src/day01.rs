use std::io::Error;
use test_case::test_case;

#[test_case("inputs/example-01" => matches Ok(1))]
pub fn puzzle1(filename: &str) -> Result<i64, Error> {
    Err(Error::other("oh no"))
}
