pub fn solve() -> i32 {
    let mut joltage = 0;
    let lines = utils::read_lines("data/day03.txt").unwrap();
    for line in lines.map_while(Result::ok) {
        let bank: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut bank_iter = bank.iter().enumerate().take(bank.len() - 1);
        let mut max = bank_iter.next().unwrap();
        for digit in bank_iter {
            if *digit.1 > *max.1 {
                max = digit;
            }

            if *digit.1 == 9 {
                break;
            }
        }

        joltage += 10 * *max.1;

        let mut bank_iter = bank.iter().enumerate().skip(max.0 + 1);
        max = bank_iter.next().unwrap();
        for digit in bank_iter {
            if *digit.1 > *max.1 {
                max = digit;
            }

            if *digit.1 == 9 {
                break;
            }
        }

        joltage += *max.1;
    }

    joltage
}
