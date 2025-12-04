pub fn solve() -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];

    let lines = utils::read_lines("data/sample04.txt").unwrap();
    for line in lines.map_while(Result::ok) {
        grid.push(line.chars().collect());
    }

    let mut count = 0;
    while let Some(removed) = take_rolls(&mut grid) {
        count += removed;
    }

    count
}

fn take_rolls(grid: &mut [Vec<char>]) -> Option<i32> {
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' && count_rolls(grid, x, y) < 4 {
                grid[y][x] = 'x';
                count += 1;
            }
        }
    }

    if count > 0 { Some(count) } else { None }
}

fn count_rolls(grid: &[Vec<char>], x: usize, y: usize) -> i32 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let char = match grid.get((y as i32 + dy) as usize) {
                Some(row) => row.get((x as i32 + dx) as usize).unwrap_or(&'.'),
                None => &'.',
            };

            if *char == '@' {
                count += 1;
            }
        }
    }
    count
}
