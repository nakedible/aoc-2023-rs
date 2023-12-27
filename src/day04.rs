use anyhow::Result;
use std::collections::HashSet;
use test_case::test_case;

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: i64,
    count: i64,
    winning: HashSet<u8>,
    numbers: HashSet<u8>,
}

fn parse_input(filename: &str) -> Result<Vec<Card>> {
    let input = std::fs::read_to_string(filename)?;
    let mut ret = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let (cardnum, cardstr) = line.split_once(':').unwrap();
        let (_, cardid) = cardnum.split_once("Card ").unwrap();
        let cardid = cardid.trim().parse().unwrap();
        let (winningstr, numbersstr) = cardstr.split_once(" |").unwrap();
        let winning = winningstr
            .as_bytes()
            .chunks(3)
            .map(|c| {
                std::str::from_utf8(c)
                    .unwrap()
                    .trim()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect();
        let numbers = numbersstr
            .as_bytes()
            .chunks(3)
            .map(|c| {
                std::str::from_utf8(c)
                    .unwrap()
                    .trim()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect();
        ret.push(Card {
            id: cardid,
            count: 1,
            winning,
            numbers,
        });
    }
    Ok(ret)
}

#[test_case("inputs/example-04-1.txt" => matches Ok(13))]
#[test_case("inputs/input-04.txt" => matches Ok(24733))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let mut total = 0;
    input.iter().for_each(|c| {
        let matching = c.winning.intersection(&c.numbers).count();
        if matching > 0 {
            total += 2i64.pow(matching as u32 - 1);
        }
    });
    Ok(total)
}

#[test_case("inputs/example-04-1.txt" => matches Ok(30))]
#[test_case("inputs/input-04.txt" => matches Ok(5422730))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let mut input = parse_input(filename)?;
    for i in 0..input.len() {
        let matching = input[i].winning.intersection(&input[i].numbers).count();
        for j in i + 1..i + 1 + matching {
            input[j].count += input[i].count;
        }
    }
    let total = input.iter().map(|c| c.count).sum();
    Ok(total)
}
