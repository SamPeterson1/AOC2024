use std::fs;

type Coord = (i32, i32);

const DIRECTIONS: [Coord; 4] = [
    (-1, 0), (0, 1), (1, 0), (0, -1)
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Guard {
    pos: Coord,
    direction_i: usize,
}

#[derive(Clone, Debug)]
struct Input {
    board: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    guard_slow: Guard,
    guard_fast: Guard,
}

fn main() {
   let mut input_1 = read_input();
   let mut input_2 = input_1.clone();

   part1(&mut input_1);
   part2(&mut input_2)
}

fn read_input() -> Input {
    let mut board: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| 
            line.chars().collect()
        ).collect();

    let mut guard_pos = (0, 0);
    let mut guard_direction_i = 0;

    board.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            let c = *c;
            if c == '^' || c == '>' || c == '<' || c == 'v' {
                guard_pos = (i as i32, j as i32);

                if c == '^' {
                    guard_direction_i = 0;
                } else if c == '>' {
                    guard_direction_i = 1;
                } else if c == 'v' {
                    guard_direction_i = 2;
                } else if c == '<' {
                    guard_direction_i = 3;
                }
            }
        });
    });

    board[guard_pos.0 as usize][guard_pos.1 as usize] = '.';

    let visited = vec![vec![false; board[0].len()]; board.len()];

    Input {
        board,
        visited,
        guard_slow: Guard {pos: guard_pos, direction_i: guard_direction_i},
        guard_fast: Guard {pos: guard_pos, direction_i: guard_direction_i}
    }
}

fn in_bounds(pos: Coord, board: &Vec<Vec<char>>) -> bool{
    pos.0 >= 0 && pos.0 < board.len() as i32 && pos.1 >= 0 && pos.1 < board[0].len() as i32
}

fn is_obstruction(pos: Coord, board: &Vec<Vec<char>>) -> bool {
    if !in_bounds(pos, board) {
        return false;
    }

    board[pos.0 as usize][pos.1 as usize] == '#'
}

fn run_guard(board: &Vec<Vec<char>>, guard: &mut Guard) {
    let mut direction = DIRECTIONS[guard.direction_i];
    let mut next_pos = (guard.pos.0 + direction.0, guard.pos.1 + direction.1);

    while is_obstruction(next_pos, board) {
        guard.direction_i = (guard.direction_i + 1) % 4;
        direction = DIRECTIONS[guard.direction_i];
        next_pos = (guard.pos.0 + direction.0, guard.pos.1 + direction.1)
    }

    guard.pos = next_pos;
}

fn part1(input: &mut Input) {
    while in_bounds(input.guard_slow.pos, &input.board) {
        input.visited[input.guard_slow.pos.0 as usize][input.guard_slow.pos.1 as usize] = true;
        run_guard(&input.board, &mut input.guard_slow);
    }

    let n_visited: usize = input.visited.iter()
        .map(|row| {
            row.iter().filter(|&e| *e).count()
        }).sum();

    println!("Part 1: {n_visited}");
}

fn has_cycle(input: &mut Input) -> bool {
    while in_bounds(input.guard_slow.pos, &input.board) && in_bounds(input.guard_fast.pos, &input.board) {
        run_guard(&input.board, &mut input.guard_slow);
        run_guard(&input.board, &mut input.guard_fast);
        run_guard(&input.board, &mut input.guard_fast);

        if input.guard_slow == input.guard_fast {
            return true;
        }
    }

    false
}

fn part2(input: &mut Input) {
    let initial_guard = input.guard_slow.clone();

    let n_cycle: u32 = (0..input.board.len()).map(|i| {
        (0..input.board[0].len()).map(|j| {
            input.guard_slow = initial_guard.clone();
            input.guard_fast = initial_guard.clone();

            if input.board[i][j] == '#' || (i as i32, j as i32) == initial_guard.pos {
                return 0;
            }

            input.board[i][j] = '#';
            let cycle = has_cycle(input) as u32;
            input.board[i][j] = '.';

            return cycle 
        }).sum::<u32>()
    }).sum();

    println!("Part 2: {n_cycle}");
}