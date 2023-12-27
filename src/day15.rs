use anyhow::Result;
use test_case::test_case;

fn calc_hash(s: &[u8]) -> u8 {
    s.iter().fold(0, |a, v| a.wrapping_add(*v).wrapping_mul(17))
}

#[test_case("inputs/example-15-1.txt" => matches Ok(1320))]
#[test_case("inputs/input-15.txt" => matches Ok(513158))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = std::fs::read(filename)?;
    let (_, input) = input.split_last().unwrap();
    let parts = input.split(|b| *b == b',').collect::<Vec<_>>();
    let ret = parts.iter().map(|v| calc_hash(v) as i64).sum::<i64>();
    Ok(ret)
}

#[derive(Debug)]
enum Instr<'a> {
    Rem(u8, &'a [u8]),
    Set(u8, &'a [u8], u8),
}

fn parse_instr(part: &str) -> Instr {
    if part.ends_with('-') {
        let part = &part.as_bytes()[..part.len() - 1];
        Instr::Rem(calc_hash(part), part)
    } else {
        let (part, lens) = part.split_once('=').unwrap();
        let part = part.as_bytes();
        let lens = lens.parse().unwrap();
        Instr::Set(calc_hash(part), part, lens)
    }
}

#[allow(dead_code)]
fn print_boxes(boxes: &[Vec<(&[u8], u8)>]) {
    for (i, box_) in boxes.iter().enumerate() {
        if !box_.is_empty() {
            println!("{}: {:?}", i, box_);
        }
    }
}

fn calc_power(boxes: &[Vec<(&[u8], u8)>]) -> i64 {
    let mut ret = 0;
    for (i, box_) in boxes.iter().enumerate() {
        for (j, (_, lens)) in box_.iter().enumerate() {
            ret += (i as i64 + 1) * (j as i64 + 1) * *lens as i64;
        }
    }
    ret
}

#[test_case("inputs/example-15-1.txt" => matches Ok(145))]
#[test_case("inputs/input-15.txt" => matches Ok(200277))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = std::fs::read_to_string(filename)?;
    let parts = input.trim().split(',').map(parse_instr).collect::<Vec<_>>();
    let mut boxes: [Vec<(&[u8], u8)>; 256] = std::array::from_fn(|_| Vec::new());
    for part in parts {
        match part {
            Instr::Rem(hash, part) => {
                boxes[hash as usize].retain(|(p, _)| p != &part);
            }
            Instr::Set(hash, part, lens) => {
                let mut found = false;
                boxes[hash as usize].iter_mut().for_each(|(p, l)| {
                    if p == &part {
                        found = true;
                        *l = lens;
                    }
                });
                if !found {
                    boxes[hash as usize].push((part, lens));
                }
            }
        }
    }
    let power = calc_power(&boxes);
    Ok(power)
}
