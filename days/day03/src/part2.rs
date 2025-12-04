const MAX_LEN: usize = 12;

pub fn solve() -> u64 {
    let mut joltage = 0;
    let lines = utils::read_lines("data/day03.txt").unwrap();
    for line in lines.map_while(Result::ok) {
        let mut bank: Vec<(usize, u64)> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .enumerate()
            .collect();

        let take_len = bank.len() - MAX_LEN + 1;
        let mut digits: Vec<(usize, u64)> = vec![];
        for _ in 0..MAX_LEN {
            let mut iter = bank.clone().into_iter().take(take_len).filter(|&(i, _)| {
                if digits.is_empty() {
                    true
                } else {
                    digits.last().unwrap().0 < i
                }
            });

            let mut max = iter.next().unwrap();
            for digit in iter {
                if digit.1 > max.1 {
                    max = digit;
                    if digit.1 == 9 {
                        continue;
                    }
                }
            }

            digits.push(max);

            if let Some(pos) = bank.iter().position(|e| e.0 == max.0) {
                bank.remove(pos);
            }
        }

        digits.sort_by(|a, b| a.0.cmp(&b.0));

        let res = digits
            .iter()
            .map(|e| e.1)
            .reduce(|acc, e| acc * 10 + e)
            .unwrap();

        joltage += res;
    }

    joltage
}
