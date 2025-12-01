pub fn solve() -> i32 {
    let mut dial = 50;
    let mut count = 0;

    let lines = utils::read_lines("data/day01.txt").unwrap();
    for line in lines.map_while(Result::ok) {
        let mut line_chars = line.chars();
        let direction = line_chars.next().unwrap();
        let amount: i32 = line_chars.collect::<String>().parse().unwrap();

        if direction == 'L' {
            dial += amount;
        } else {
            dial -= amount;
        }

        dial %= 100;

        if dial == 0 {
            count += 1;
        }
    }

    count
}
