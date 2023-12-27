use anyhow::Result;
use test_case::test_case;

#[derive(Debug, Default)]
struct Game {
    red: i64,
    blue: i64,
    green: i64,
}

fn parse_game(gamestr: &str) -> Game {
    gamestr.split(", ").fold(Game::default(), |game, colorcount| {
        let (count, color) = colorcount.split_once(' ').unwrap();
        let count = count.parse().unwrap();
        match color {
            "red" => Game { red: count, ..game },
            "blue" => Game { blue: count, ..game },
            "green" => Game { green: count, ..game },
            _ => panic!("Unknown color {}", color),
        }
    })
}

fn parse_input(filename: &str) -> Result<Vec<(i64, Vec<Game>)>> {
    let input = std::fs::read_to_string(filename)?;
    let ret = input.lines().map(|line| {
        let (gamenum, gamesstr) = line.split_once(": ").unwrap();
        let gameid = gamenum.strip_prefix("Game ").unwrap().parse().unwrap();
        let games = gamesstr.split("; ").map(parse_game).collect();
        (gameid, games)
    }).collect();
    Ok(ret)
}

#[test_case("inputs/example-02-1.txt" => matches Ok(8))]
#[test_case("inputs/input-02.txt" => matches Ok(2331))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let limit = Game {
        red: 12,
        green: 13,
        blue: 14,
    };
    let ret = input
        .iter()
        .filter(|(_, games)| {
            games.iter().all(|game| {
                game.red <= limit.red && game.green <= limit.green && game.blue <= limit.blue
            })
        })
        .map(|(gameid, _)| gameid)
        .sum::<i64>();
    Ok(ret)
}

#[test_case("inputs/example-02-1.txt" => matches Ok(2286))]
#[test_case("inputs/input-02.txt" => matches Ok(71585))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let ret = input
        .iter()
        .map(|(_, games)| {
            let (r, g, b) = games.iter().fold((0, 0, 0), |(red, green, blue), game| {
                (
                    std::cmp::max(red, game.red),
                    std::cmp::max(green, game.green),
                    std::cmp::max(blue, game.blue),
                )
            });
            r * g * b
        })
        .sum::<i64>();
    Ok(ret)
}
