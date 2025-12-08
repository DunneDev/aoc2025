use std::ops::RangeInclusive;

pub fn solve() -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];

    let mut lines = utils::read_lines("data/day05.txt").unwrap();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut range = line.split('-').map(|e| e.parse::<u64>().unwrap());
        let low = range.next().unwrap();
        let high = range.next().unwrap();

        ranges.push(low..=high);
    }

    let mut fresh_count = 0;

    while let Some(Ok(line)) = lines.next() {
        let id: u64 = line.parse().unwrap();

        for range in ranges.iter() {
            if range.contains(&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}
