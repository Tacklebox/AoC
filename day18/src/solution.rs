#[derive(PartialEq,Debug)]
enum Operator {
    Add,
    Multiply,
    LeftParen,
    RightParen,
}

#[derive(PartialEq,Debug)]
enum MathTrain {
    Op(Operator),
    Number(i64),
}

fn parse_forward(input: &mut impl Iterator<Item = char>) -> i64 {
    let mut result = 0;
    let mut current_operator = Operator::Add;
    while let Some(c) = input.next() {
        match c {
            d @ '0'..='9' => match current_operator {
                Operator::Add => result += d.to_digit(10).unwrap() as i64,
                Operator::Multiply => result *= d.to_digit(10).unwrap() as i64,
                _ => panic!(),
            },
            '+' => current_operator = Operator::Add,
            '*' => current_operator = Operator::Multiply,
            '(' => match current_operator {
                Operator::Add => result += parse_forward(input),
                Operator::Multiply => result *= parse_forward(input),
                _ => panic!(),
            },
            ')' => return result,
            ' ' => continue,
            _ => panic!("Unknown input character {}", c),
        }
    }
    result
}

fn parse_tokens(input: &mut impl Iterator<Item = char>) -> Vec<MathTrain> {
    input.filter_map(|c| match c {
        d @ '0'..='9' => Some(MathTrain::Number(d.to_digit(10).unwrap() as i64)),
        '+' => Some(MathTrain::Op(Operator::Add)),
        '*' => Some(MathTrain::Op(Operator::Multiply)),
        '(' => Some(MathTrain::Op(Operator::LeftParen)),
        ')' => Some(MathTrain::Op(Operator::RightParen)),
        ' ' => None,
        _ => panic!("invalid input character {}", c),
    }).collect()
}

fn infix_to_rpn(infix: Vec<MathTrain>) -> Vec<MathTrain> {
    let mut operator_stack: Vec<Operator> = Vec::new();
    let mut output: Vec<MathTrain> = Vec::new();
    let mut infix_iter = infix.into_iter();
    while let Some(c) = infix_iter.next() {
        match c {
            MathTrain::Number(_) => output.push(c),
            MathTrain::Op(op @ Operator::Add) | MathTrain::Op(op @ Operator::Multiply) => {
                while operator_stack.len() > 0
                    && !(op == Operator::Add
                        && *operator_stack.last().unwrap() == Operator::Multiply)
                    && *operator_stack.last().unwrap() != Operator::LeftParen
                {
                    output.push(MathTrain::Op(operator_stack.pop().unwrap()));
                }
                operator_stack.push(op);
            }
            MathTrain::Op(Operator::LeftParen) => operator_stack.push(Operator::LeftParen),
            MathTrain::Op(Operator::RightParen) => {
                let mut matched = false;
                while let Some(op) = operator_stack.pop() {
                    if op != Operator::LeftParen {
                        output.push(MathTrain::Op(op));
                    } else {
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    panic!("Mismatched Parenthesis!");
                }
            }
        };
    }
    while let Some(op) = operator_stack.pop() {
        output.push(MathTrain::Op(op));
    }
    output
}

fn execute_rpn(rpn: Vec<MathTrain>) -> i64 {
    let mut execution_stack: Vec<i64> = Vec::new();
    for el in rpn.into_iter() {
        match el {
            MathTrain::Number(n) => execution_stack.push(n),
            MathTrain::Op(Operator::Add) => {
                let operand1 = execution_stack.pop().unwrap();
                let operand2 = execution_stack.pop().unwrap();
                execution_stack.push(operand1 + operand2);
            }
            MathTrain::Op(Operator::Multiply) => {
                let operand1 = execution_stack.pop().unwrap();
                let operand2 = execution_stack.pop().unwrap();
                execution_stack.push(operand1 * operand2);
            }
            _ => panic!("Invalid RPN")
        }
    }
    execution_stack.pop().unwrap()
}

pub fn part1(input: impl Iterator<Item = String>) -> i64 {
    let mut sum = 0;
    for line in input {
        sum += parse_forward(&mut line.chars());
    }
    sum
}

pub fn part2(input: impl Iterator<Item = String>) -> i64 {
    let mut sum = 0;
    for line in input {
        let parsed = parse_tokens(&mut line.chars());
        let rpn = infix_to_rpn(parsed);
        let result = execute_rpn(rpn);
        sum += result;
    }
    sum
}

mod test {
    #[test]
    fn example_part1() {
        let input = vec![String::from("1 + 2 * 3 + 4 * 5 + 6")];
        assert_eq!(super::part1(input.into_iter()), 71);
        let input = vec![String::from("2 * 3 + (4 * 5)")];
        assert_eq!(super::part1(input.into_iter()), 26);
        let input = vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")];
        assert_eq!(super::part1(input.into_iter()), 437);
        let input = vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")];
        assert_eq!(super::part1(input.into_iter()), 12240);
        let input = vec![String::from(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )];
        assert_eq!(super::part1(input.into_iter()), 13632);
    }

    #[test]
    fn parse_tokens_test() {
        use super::{MathTrain, Operator, parse_tokens};

        let mut input = "1 + 2 * 3 + 4 * 5 + 6".chars();
        assert_eq!(parse_tokens(&mut input), vec![
                   MathTrain::Number(1),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(2),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(3),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(4),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(5),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(6),
        ]);
        let mut input = "2 * 3 + (4 * 5)".chars();
        assert_eq!(parse_tokens(&mut input), vec![
                   MathTrain::Number(2),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(3),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Op(Operator::LeftParen),
                   MathTrain::Number(4),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(5),
                   MathTrain::Op(Operator::RightParen),
        ]);
    }

    #[test]
    fn infix_to_rpn_test() {
        use super::{MathTrain, Operator, infix_to_rpn};

        let input = vec![
                   MathTrain::Number(1),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(2),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(3),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(4),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(5),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(6),
        ];
        assert_eq!(infix_to_rpn(input), vec![
                   MathTrain::Number(1),
                   MathTrain::Number(2),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Number(3),
                   MathTrain::Number(4),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(5),
                   MathTrain::Number(6),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Op(Operator::Multiply),
        ]);

        let input = vec![
                   MathTrain::Number(2),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(3),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Op(Operator::LeftParen),
                   MathTrain::Number(4),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Number(5),
                   MathTrain::Op(Operator::RightParen),
        ];
        assert_eq!(infix_to_rpn(input), vec![
                   MathTrain::Number(2),
                   MathTrain::Number(3),
                   MathTrain::Number(4),
                   MathTrain::Number(5),
                   MathTrain::Op(Operator::Multiply),
                   MathTrain::Op(Operator::Add),
                   MathTrain::Op(Operator::Multiply),
        ]);
    }

    #[test]
    fn example_part2() {
        let input = vec![String::from("1 + 2 * 3 + 4 * 5 + 6")];
        assert_eq!(super::part2(input.into_iter()), 231);
        let input = vec![String::from("2 * 3 + (4 * 5)")];
        assert_eq!(super::part2(input.into_iter()), 46);
        let input = vec![String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")];
        assert_eq!(super::part2(input.into_iter()), 1445);
        let input = vec![String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")];
        assert_eq!(super::part2(input.into_iter()), 669060);
        let input = vec![String::from(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        )];
        assert_eq!(super::part2(input.into_iter()), 23340);
    }
}
