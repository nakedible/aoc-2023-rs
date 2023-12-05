use anyhow::Result;
use rangemap::RangeMap;
use test_case::test_case;

fn clamped_overlapping(
    map: &RangeMap<i64, i64>,
    range: &std::ops::Range<i64>,
) -> Vec<(std::ops::Range<i64>, i64)> {
    map.overlapping(range)
        .map(|(r, v)| {
            (
                std::cmp::max(r.start, range.start)..std::cmp::min(r.end, range.end),
                *v,
            )
        })
        .collect()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_to_soil: Vec<(i64, i64, i64)>,
    soil_to_fertilizer: Vec<(i64, i64, i64)>,
    fertilizer_to_water: Vec<(i64, i64, i64)>,
    water_to_light: Vec<(i64, i64, i64)>,
    light_to_temperature: Vec<(i64, i64, i64)>,
    temperature_to_humidity: Vec<(i64, i64, i64)>,
    humidity_to_location: Vec<(i64, i64, i64)>,
}

fn build_ranges(data: &str) -> Vec<(i64, i64, i64)> {
    data.lines()
        .skip(1)
        .map(|line| {
            let mut parts = line.split(" ").map(|v| v.parse::<i64>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

fn parse_input(filename: &str) -> Result<Almanac> {
    let input = std::fs::read_to_string(filename)?;
    let [seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location] =
        input.split("\n\n").collect::<Vec<&str>>()[..]
    else {
        return Err(anyhow::anyhow!("Invalid input"));
    };
    let seeds = seeds
        .split(" ")
        .skip(1)
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let seed_to_soil = build_ranges(seed_to_soil);
    let soil_to_fertilizer = build_ranges(soil_to_fertilizer);
    let fertilizer_to_water = build_ranges(fertilizer_to_water);
    let water_to_light = build_ranges(water_to_light);
    let light_to_temperature = build_ranges(light_to_temperature);
    let temperature_to_humidity = build_ranges(temperature_to_humidity);
    let humidity_to_location = build_ranges(humidity_to_location);
    return Ok(Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    });
}

// fn map_naive(base: &[u8; 100], ranges: &Vec<(i64, i64, i64)>) -> [u8; 100] {
//     let mut ret = base.clone();
//     for (dst, src, len) in ranges {
//         for i in 0..*len {
//             ret[(src+i) as usize] = base[(dst+i) as usize] as u8;
//         }
//     }
//     ret
// }

// fn naive_to_rangemap(naive: &[u8; 100]) -> RangeMap<i64, i64> {
//     let mut ret = RangeMap::new();
//     for i in 0..naive.len() {
//         ret.insert(i as i64..(i+1) as i64, naive[i] as i64 - i as i64);
//     }
//     ret
// }

fn map_ranges(base: &RangeMap<i64, i64>, ranges: &Vec<(i64, i64, i64)>) -> RangeMap<i64, i64> {
    let mut ret = base.clone();
    for (dst, src, len) in ranges {
        let offset = dst - src;
        let srcrange = src + 0..src + len;
        for (srcchunk, _) in clamped_overlapping(base, &srcrange) {
            let dstrange = srcchunk.start + offset..srcchunk.end + offset;
            for (dstchunk, dstv) in clamped_overlapping(base, &dstrange) {
                let srcdest = dstchunk.start - offset..dstchunk.end - offset;
                ret.insert(srcdest, dstv + offset);
            }
        }
        for gap in base.gaps(&srcrange) {
            let srcdest = gap.start..gap.end;
            ret.insert(srcdest, offset);
        }
    }
    ret
}

fn min_ranges(base: &RangeMap<i64, i64>, ranges: &Vec<(i64, i64)>) -> i64 {
    let mut ret = i64::MAX;
    for (src, len) in ranges {
        let srcrange = src + 0..src + len;
        for (srcchunk, _) in clamped_overlapping(base, &srcrange) {
            ret = std::cmp::min(ret, srcchunk.start + base.get(&srcchunk.start).unwrap());
        }
        for gap in base.gaps(&srcrange) {
            ret = std::cmp::min(ret, gap.start);
        }
    }
    ret
}

fn compile_almanac(input: &Almanac) -> RangeMap<i64, i64> {
    //let mut naive: [u8; 100] = std::array::from_fn(|i| i as u8);
    let mut map = RangeMap::new();
    for ranges in [
        &input.humidity_to_location,
        &input.temperature_to_humidity,
        &input.light_to_temperature,
        &input.water_to_light,
        &input.fertilizer_to_water,
        &input.soil_to_fertilizer,
        &input.seed_to_soil,
    ] {
        //naive = map_naive(&naive, ranges);
        map = map_ranges(&map, ranges);
    }
    map
}

#[test_case("inputs/example-05-1.txt" => matches Ok(35))]
#[test_case("inputs/input-05.txt" => matches Ok(157211394))]
pub fn puzzle1(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let map = compile_almanac(&input);
    let mapped = input
        .seeds
        .iter()
        .map(|v| v + map.get(v).unwrap_or(&0))
        .collect::<Vec<i64>>();
    let ret = mapped.into_iter().min().unwrap();
    Ok(ret)
}

#[test_case("inputs/example-05-1.txt" => matches Ok(46))]
#[test_case("inputs/input-05.txt" => matches Ok(50855035))]
pub fn puzzle2(filename: &str) -> Result<i64> {
    let input = parse_input(filename)?;
    let map = compile_almanac(&input);
    let seeds = input
        .seeds
        .chunks(2)
        .map(|s| (s[0], s[1]))
        .collect::<Vec<(i64, i64)>>();
    let ret = min_ranges(&map, &seeds);
    Ok(ret)
}
