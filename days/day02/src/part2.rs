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
        'test_number: for num in range[0]..=range[1] {
            let num_str = num.to_string();
            let num_len = num_str.len();

            'test_section: for i in 1..=num_len / 2 {
                if num_len % i != 0 {
                    continue;
                }

                let comp_val = &num_str[0..i];
                for j in 1..num_len / i {
                    let start = j * i;
                    let end = (j + 1) * i;
                    if comp_val != &num_str[start..end] {
                        continue 'test_section;
                    }
                }

                count += num;
                continue 'test_number;
            }
        }
    }

    count
}
