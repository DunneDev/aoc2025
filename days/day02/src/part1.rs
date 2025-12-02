use std::fs;

pub fn solve() -> u64 {
    let input: Vec<Vec<u64>> = fs::read_to_string("data/day02.txt")
        .unwrap()
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|val| val.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let mut count = 0u64;
    for range in input {
        for num in range[0]..=range[1] {
            let num_str = num.to_string();
            let num_len = num_str.len();

            if num_len % 2 != 0 {
                continue;
            }

            let half_len = num_len / 2;
            if num_str[0..half_len] == num_str[half_len..num_len] {
                count += num;
            }
        }
    }

    count
}
