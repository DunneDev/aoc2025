#[derive(Debug)]
enum Operation {
    Add,
    Mult,
    None,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn new(number: u64) -> Self {
        Problem {
            numbers: vec![number],
            operation: Operation::None,
        }
    }

    fn solve(self) -> u64 {
        let reduce_fn: fn(u64, u64) -> u64 = match self.operation {
            Operation::Add => |sum, number| sum + number,
            Operation::Mult => |product, number| product * number,
            _ => |x, _| x,
        };

        self.numbers.into_iter().reduce(reduce_fn).unwrap()
    }
}

pub fn solve() -> u64 {
    let mut problems: Vec<Problem> = vec![];

    let mut lines = utils::read_lines("data/day06.txt").unwrap();
    while let Some(Ok(line)) = lines.next() {
        if line.chars().next().unwrap().is_ascii_punctuation() {
            let mut operation_iter = line.chars().filter_map(|c| match c {
                '*' => Some(Operation::Mult),
                '+' => Some(Operation::Add),
                _ => None,
            });
            let mut problem_iter = problems.iter_mut();
            while let (Some(operation), Some(problem)) =
                (operation_iter.next(), problem_iter.next())
            {
                problem.operation = operation;
            }

            break;
        }
        for (index, number) in line
            .split(' ')
            .filter_map(|n| n.trim().parse::<u64>().ok())
            .enumerate()
        {
            match problems.get_mut(index) {
                Some(problem) => problem.numbers.push(number),
                None => problems.push(Problem::new(number)),
            }
        }
    }

    problems
        .into_iter()
        .fold(0u64, |total, problem| total + problem.solve())
}
