const MAX_LEN: usize = 12;

pub fn solve() -> u64 {
    let mut joltage = 0;
    let lines = utils::read_lines("data/day03.txt").unwrap();
    for line in lines.map_while(Result::ok) {
        let bank: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut stack: Vec<u64> = vec![];
        let mut max = 0u64;
        let mut max_digit = 0u64;
        for i in 0..=bank.len() - MAX_LEN {
            if bank[i] <= max_digit {
                continue;
            }
            max_digit = bank[i];

            let res = traverse(&bank, &mut stack, i);
            if res > max {
                max = res;
            }
        }

        joltage += max;
    }

    joltage
}

fn traverse(bank: &Vec<u64>, stack: &mut Vec<u64>, index: usize) -> u64 {
    stack.push(bank[index]);
    if stack.len() != MAX_LEN {
        let mut max = 0u64;
        let mut max_digit = 0u64;
        for i in (index + 1)..=(bank.len() + stack.len() - MAX_LEN) {
            if bank[i] <= max_digit {
                continue;
            }
            max_digit = bank[i];

            let res = traverse(bank, stack, i);
            if res > max {
                max = res;
            }
        }

        stack.pop();
        max
    } else {
        let joltage = stack
            .iter()
            .map(|e| e.to_string())
            .reduce(|mut acc, e| {
                acc.push_str(&e);
                acc
            })
            .unwrap()
            .parse()
            .unwrap();

        stack.pop();
        joltage
    }
}
