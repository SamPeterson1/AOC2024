use std::fs;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add, Mult, Concat
}

#[derive(Debug)]
struct Equation {
    target: u64,
    values: Vec<u64>,
}

struct Input {
    equations: Vec<Equation>
}

static OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mult, Operator::Concat];

fn main() {
    let input = read_input();

    part1(&input);
    part2(&input);
}

fn read_input() -> Input {
    let equations: Vec<Equation> = fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            let mut split_iter = line.split(": ");
            
            let target = split_iter.next().unwrap()
                .parse().unwrap();
            let values = split_iter.next().unwrap().split(" ")
                .map(|e| e.parse().unwrap())
                .collect();

            Equation {target, values}
        }).collect();

    Input {equations}
}

fn check_equation(equation: &Equation, n_operator_types: usize) -> bool {
    let n_operators = equation.values.len() - 1;
    let mut operators: Vec<Operator> = Vec::new();

    let n_states = n_operator_types.pow(n_operators as u32);

    for i in 0..n_states {
        let mut j = i;
        operators.clear();

        for _ in 0..n_operators {
            operators.push(OPERATORS[j % n_operator_types]);
            j /= n_operator_types;
        }

        let mut result = equation.values[0];

        for (j, operator) in operators.iter().enumerate() {
            match operator {
                Operator::Add => result += equation.values[j + 1],
                Operator::Mult => result *= equation.values[j+ 1],
                Operator::Concat => result = (result.to_string() + &equation.values[j + 1].to_string()).parse().unwrap(),
            }
        }

        if result == equation.target {
            return true;
        }
    }

    false
}

fn part1(input: &Input) {
    let ans: u64 = input.equations.iter().filter(|&equation| {
        check_equation(equation, 2)
    }).map(|equation| {
        equation.target
    }).sum();

    println!("Part 1: {ans}");
}

fn part2(input: &Input) {
    let ans: u64 = input.equations.iter().filter(|&equation| {
        check_equation(equation, 3)
    }).map(|equation| {
        equation.target
    }).sum();

    println!("Part 2: {ans}");
}
