use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Update {
    pages: Vec<i32>,
    page_indices: HashMap<i32, usize>
}

#[derive(Debug)]
struct Input {
    orderings: Vec<(i32, i32)>,
    updates: Vec<Update>
}

fn main() {
    let mut input = read_input();

    part1(&mut input);
    part2(&mut input);
}

fn read_input() -> Input {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines();

    let mut orderings = Vec::new();

    loop {
        let line = lines.next().unwrap();

        if line == "" {
            break;
        }

        let mut ordering_split = line.split("|");
        let left_ordering: i32 = ordering_split.next().unwrap().parse().unwrap();
        let right_ordering: i32 = ordering_split.next().unwrap().parse().unwrap();

        orderings.push((left_ordering, right_ordering));
    }


    let updates = lines.map(|line| {
        let mut update = Update {
            page_indices: HashMap::new(),
            pages: Vec::new()
        };

        line.split(",").enumerate().for_each(|(i, e)| {
            let q = e.parse::<i32>().unwrap();
            update.page_indices.insert(q, i);
            update.pages.push(q);
        });

        update
    }).collect();

    Input {
        orderings,
        updates,
    }
}

fn validate_orderings(update: &mut Update, orderings: &[(i32, i32)]) -> bool {
    for &ordering in orderings.iter() {
        let left_index = update.page_indices.get(&ordering.0);
        let right_index = update.page_indices.get(&ordering.1);

        if let (Some(&left_index), Some(&right_index)) = (left_index, right_index) {
            if left_index > right_index {
                return false;
            }
        }
        
    }

    true
}

fn compare(a: i32, b: i32, orderings: &[(i32, i32)]) -> i32 {
    for ordering in orderings.iter() {
        if ordering.0 == a && ordering.1 == b {
            return -1;
        } else if ordering.0 == b && ordering.1 == a {
            return 1;
        }
    }

    return 0;
}

fn sort(vec: &mut Vec<i32>, orderings: &[(i32, i32)]) {
    for i in 0..vec.len() {
        for j in 0..(vec.len() - i - 1) {
            if compare(vec[j], vec[j + 1], orderings) < 0 {
                let tmp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = tmp;
            }
        }
    }
}

fn get_middle_page(update: &Update) -> i32 {
    update.pages[update.pages.len() / 2]
}

fn part1(input: &mut Input) {
    let mut result = 0;

    for update in input.updates.iter_mut() {
        if validate_orderings(update, &input.orderings) {
            result += get_middle_page(update);
        }
    }

    println!("Part 1: {result}");
}

fn part2(input: &mut Input) {
    let mut result = 0;

    for update in input.updates.iter_mut() {
        if !validate_orderings(update, &input.orderings) {
            sort(&mut update.pages, &input.orderings);
            result += get_middle_page(update);
        }
    }

    println!("Part 2: {result}");
}