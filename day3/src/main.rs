use std::fs;
use std::iter;

#[derive(Debug)]
enum ParserState {
    Mul,
    LeftParen,
    LeftArg,
    Comma,
    RightArg,
    RightParen,
    Done
}

struct Parser {
    enabled: bool,
    input: Vec<char>,
    index: usize,
    left: i32,
    right: i32,
    state: ParserState
}

impl Parser {

    fn reset(&mut self) {
        self.reset_state();
        self.index = 0;
        self.left = 0;
        self.right = 0;
    }
    
    fn reset_state(&mut self) {
        self.state = ParserState::Mul;
    }

    fn has_next(&self) -> bool {
        self.index < self.input.len()
    }
    
    fn has_peek(&self) -> bool {
        (self.index + 1) < self.input.len()
    }

    fn next(&mut self) -> Option<char> {
        let cur = self.cur();
        self.index += 1;

        cur
    }

    fn peek(&mut self) -> Option<char> {
        if !self.has_peek() {
            return None;
        }

        Some(self.input[self.index + 1])
    }

    fn cur(&self) -> Option<char> {
        if !self.has_next() {
            return None;
        }

        Some(self.input[self.index])

    }

}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().chars().collect();

    let mut parser = Parser {
        enabled: true,
        input,
        index: 0,
        left: 0,
        right: 0,
        state: ParserState::Mul
    };
   
    part1(&mut parser);
    parser.reset();
    part2(&mut parser);
}

fn update_enabled(parser: &mut Parser) -> Option<()> {
    for c in "do".chars() {
        if parser.cur()? != c {
            return None;
        } else {
            parser.next();
        }
    }

    let mut do_command = true;
    let mut dont_command = false;
    
    for c in "()".chars() {
        if parser.cur()? != c {
            do_command = false;
            break;
        } else {
            parser.next();
        }
    }

    if !do_command {
        dont_command = true;
        for c in "n't()".chars() {
            if parser.cur()? != c {
                do_command = false;
                dont_command = true;
                break;
            } else {
                parser.next();
            }
        }
    }

    println!("{:?}, {:?}", do_command, dont_command);

    if dont_command {
        parser.enabled = false;
    } else if do_command {
        parser.enabled = true;
    }

    Some(())
}

fn expect_mul(parser: &mut Parser) -> Option<()> {
    for c in "mul".chars() {
        if parser.next()? != c {
           parser.reset_state();
           return None;
        }
    }

    parser.state = ParserState::LeftParen;
    Some(())
} 

fn expect_left_paren(parser: &mut Parser) -> Option<()> {
    if parser.cur()? == '(' {
        parser.next();
        parser.state = ParserState::LeftArg;
    } else {
        parser.reset_state();
    }

    Some(())
}

fn read_int(parser: &mut Parser) -> Option<i32> {
    let mut c = parser.cur()?;
    let mut value: u32 = 0;
    let mut n_digits = 0;

    if !c.is_ascii_digit() {
        parser.reset_state();
        return None;
    }

    while (c.is_ascii_digit() && n_digits < 3) {
        parser.next();
        n_digits += 1;
        value = 10 * value + c.to_digit(10).unwrap();
        c = parser.cur()?
    }

    Some(value.try_into().unwrap())
}

fn expect_left_arg(parser: &mut Parser) -> Option<()> {
    let value = read_int(parser)?;

    parser.state = ParserState::Comma;
    parser.left = value;
    Some(())
}

fn expect_comma(parser: &mut Parser) -> Option<()> {
    if parser.cur()? == ',' {
        parser.next();
        parser.state = ParserState::RightArg
    } else {
        parser.reset_state();
    }

    Some(())
}

fn expect_right_arg(parser: &mut Parser) -> Option<()> {
    let value = read_int(parser)?;

    parser.state = ParserState::RightParen;
    parser.right = value;
    Some(())
}

fn expect_right_paren(parser: &mut Parser) -> Option<()> {
    if parser.cur()? == ')' {
        parser.next();
        parser.state = ParserState::Done;
    } else {
        parser.reset_state();
    }

    Some(())
}

fn next_result(parser: &mut Parser, use_enabled: bool) -> Option<i32> {
    while parser.has_next() {
        if use_enabled {
            update_enabled(parser);
        }

        if parser.enabled {
            match parser.state {
                ParserState::Mul => expect_mul(parser),
                ParserState::LeftParen => expect_left_paren(parser),
                ParserState::LeftArg => expect_left_arg(parser),
                ParserState::Comma => expect_comma(parser),
                ParserState::RightArg => expect_right_arg(parser),
                ParserState::RightParen => expect_right_paren(parser),
                ParserState::Done => {
                    let result = parser.left * parser.right;
                    parser.reset_state();
                    return Some(result);
                }
            };
        } else {
            parser.next();
        }

        println!("State: {:?}, Index: {:?}, Character: {:?}, Enabled: {:?}", parser.state, parser.index, parser.cur(), parser.enabled);
    }

    None
}

fn part1(parser: &mut Parser) {
    let mut result_sum = 0;

    while let Some(next_result) = next_result(parser, false) {
        result_sum += next_result;
    }

    println!("Part 1: {result_sum}");
}

fn part2(parser: &mut Parser) {
    let mut result_sum = 0;

    while let Some(next_result) = next_result(parser, true) {
        result_sum += next_result;
    }

    println!("Part 2: {result_sum}");
}
