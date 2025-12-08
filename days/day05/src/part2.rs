#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    low: u64,
    high: u64,
}

impl Range {
    fn new(line: &str) -> Self {
        let mut range_iter = line.split('-').map(|e| e.parse::<u64>().unwrap());
        let low = range_iter.next().unwrap();
        let high = range_iter.next().unwrap();

        Self { low, high }
    }
}

enum RangeInsertLocation {
    None,
    Before(usize),
    After,
}

pub fn solve() -> u64 {
    let mut lines = utils::read_lines("data/day05.txt").unwrap();
    let line = lines.next().unwrap().unwrap();

    let mut ranges: Vec<Range> = vec![];
    let range = Range::new(&line);
    ranges.push(range);

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let new_range = Range::new(&line);
        let mut range_added = false;
        let mut location = RangeInsertLocation::None;

        // println!("{:?}", ranges);
        // println!("{:?}", new_range);

        for (i, range) in ranges.iter_mut().enumerate() {
            if new_range.high < range.low {
                location = RangeInsertLocation::Before(i);
                range_added = true;
            } else if new_range.high <= range.high {
                range.low = new_range.low;
                range_added = true;
            } else if new_range.low < range.low {
                range.low = new_range.low;
                range.high = new_range.high;
                range_added = true;
            } else if new_range.low <= range.high {
                range.high = new_range.high;
                range_added = true;
            }
        }

        if !range_added {
            location = RangeInsertLocation::After;
        }

        match location {
            RangeInsertLocation::Before(insert_pos) => ranges.insert(insert_pos, new_range),
            RangeInsertLocation::After => ranges.push(new_range),
            _ => (),
        }
    }

    let mut total = 0u64;

    ranges.sort();
    for range in ranges.windows(2) {
        if range[0].low > range[1].low {
            println!("wtffff")
        }
    }

    // println!("{:?}", ranges);
    merge_ranges(&mut ranges);
    // println!("{:?}", ranges);

    for range in ranges {
        total += range.high - range.low + 1;
    }

    total
}

#[derive(Debug)]
enum MergeState {
    NoMerge,
    Merge(usize),
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    loop {
        let mut merge_state = MergeState::NoMerge;

        let mut ranges_iter = ranges.iter_mut().enumerate().peekable();
        let mut curr_enum = ranges_iter.next().unwrap();

        while let Some((i, next_range)) = ranges_iter.peek() {
            let (_, curr_range) = curr_enum;

            if curr_range.high > next_range.low {
                curr_range.high = next_range.high;
                merge_state = MergeState::Merge(*i);
                break;
            }

            curr_enum = ranges_iter.next().unwrap();
        }

        match merge_state {
            MergeState::NoMerge => return,
            MergeState::Merge(index) => ranges.remove(index),
        };
    }
}
