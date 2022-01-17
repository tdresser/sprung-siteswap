use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "normalize.pest"]
struct SSParser;

#[derive(Debug)]
pub struct Pattern {
    pub zip_positions: Vec<(Position, Position)>,
    pub nonzip_positions: Vec<(Position, Position)>,
    pub siteswap: Vec<u32>,
}

fn get_advanced_position_str(a: &Position) -> &str {
    return match a {
        Position::BottomNatural => "B",
        Position::BottomOpposite => "b",
        Position::TopNatural => "T",
        Position::TopOpposite => "t",
    };
}

fn get_position_str((a, b): &(Position, Position)) -> String {
    return match (a, b) {
        (Position::BottomNatural, &Position::BottomNatural) => "n".to_string(),
        (Position::BottomOpposite, &Position::BottomOpposite) => "c".to_string(),
        (Position::TopNatural, &Position::TopNatural) => "i".to_string(),
        (Position::TopOpposite, &Position::TopOpposite) => "ci".to_string(),
        (a, b) => get_advanced_position_str(a).to_owned() + get_advanced_position_str(b),
    };
}

// Collapses repetition. e.g., cici -> ci.
fn collapse_positions(positions: &Vec<(Position, Position)>) -> Vec<(Position, Position)> {
    if positions.len() == 0 {
        return vec![];
    }
    for n in (1..positions.len() + 1).rev() {
        println!("N {}", n);
        // Can we divide |positions| into chunks of length n?
        if positions.len() % n != 0 {
            continue;
        }
        // Are all chunks equal?
        let mut chunks = positions.chunks_exact(positions.len() / n);
        let first = chunks.nth(0).unwrap();
        if chunks.all(|x| x == first) {
            // All chunks are equal, we're done.
            return first.to_vec();
        }
    }
    println!("{:?}", positions);
    unreachable!();
}

impl Pattern {
    fn normalize(&mut self) {
        self.zip_positions = collapse_positions(&self.zip_positions);
    }

    pub fn get_canonical_form(&mut self) -> String {
        self.normalize();
        let mut result = "".to_string();
        for zip_position in &self.zip_positions {
            result = format!("{}{}z", result, get_position_str(zip_position));
        }
        result += "S";
        assert!(self.nonzip_positions.len() == self.siteswap.len());

        let mut current_position = "n".to_string();
        for i in 0..self.nonzip_positions.len() {
            let new_position = get_position_str(&self.nonzip_positions[i]);
            let digit = char::from_digit(self.siteswap[i], 16).unwrap();
            if current_position == new_position {
                result = format!("{}{}", result, digit,);
            } else {
                result = format!("{}{}{}", result, new_position, digit,);
            }
            current_position = new_position;
        }

        return result;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Position {
    BottomNatural,
    BottomOpposite,
    TopNatural,
    TopOpposite,
}

fn map_advanced_position(pair: Pair<Rule>) -> Position {
    return match pair.as_rule() {
        Rule::bottom_natural => Position::BottomNatural,
        Rule::bottom_opposite => Position::BottomOpposite,
        Rule::top_natural => Position::TopNatural,
        Rule::top_opposite => Position::TopOpposite,
        _ => unreachable!(),
    };
}

fn parse_position(pair: Pair<Rule>) -> (Position, Position) {
    return match pair.as_rule() {
        Rule::crossed => (Position::BottomOpposite, Position::BottomOpposite),
        Rule::inverted => (Position::TopNatural, Position::TopNatural),
        Rule::crossed_inverted => (Position::TopOpposite, Position::TopOpposite),
        Rule::natural => (Position::BottomNatural, Position::BottomNatural),
        Rule::position_pair => {
            let mut positions = pair.into_inner();
            let p1 = positions.next().unwrap();
            let p2 = positions.next().unwrap();
            assert!(positions.next() == None);
            return (map_advanced_position(p1), map_advanced_position(p2));
        }
        _ => unreachable!(),
    };
}

fn parse_zip_positions(pairs: &mut Pairs<Rule>) -> Vec<(Position, Position)> {
    let mut positions: Vec<(Position, Position)> = vec![];
    loop {
        match pairs.peek() {
            // Zip positions are never last.
            None => unreachable!(),
            Some(pair) => {
                if pair.as_rule() == Rule::zip_position {
                    pairs.next();
                    let mut inner = pair.into_inner();
                    let position = inner.next();
                    match position {
                        None => unreachable!(),
                        Some(pos) => positions.push(parse_position(pos)),
                    }
                    assert!(inner.next() == None);
                } else {
                    break;
                }
            }
        }
    }
    return positions;
}

fn print_pairs(pairs: &mut Pairs<Rule>) {
    let mut s = "".to_string();
    let mut r = "".to_string();
    for p in pairs {
        s += p.as_str();
        r = format!("{}\n{}", r, p);
    }
    println!("{}", s);
    println!("{}", r);
}

fn parse_positioned_digits(pattern: &mut Pattern, pairs: &mut Pairs<Rule>) {
    let mut positions: Vec<(Position, Position)> = vec![];
    let mut siteswap: Vec<u32> = vec![];
    loop {
        match pairs.peek() {
            None => break,
            Some(pair) => {
                if pair.as_rule() == Rule::positioned_digit {
                    pairs.next();
                    let mut inner = pair.into_inner();
                    let mut position_or_digit = inner.next().unwrap();
                    if position_or_digit.as_rule() == Rule::position {
                        positions.push(parse_position(
                            position_or_digit.into_inner().next().unwrap(),
                        ));
                        position_or_digit = inner.next().unwrap();
                    } else {
                        positions.push((Position::BottomNatural, Position::BottomNatural));
                    }
                    let digit = position_or_digit;

                    let s = digit.as_str();
                    assert!(s.len() == 1);
                    let c = s.chars().nth(0).unwrap();
                    siteswap.push(c.to_digit(16).unwrap());
                    assert!(inner.next() == None);
                } else {
                    break;
                }
            }
        }
    }
    pattern.siteswap = siteswap;
    pattern.nonzip_positions = positions;
}

pub fn parse(s: &str) -> Pattern {
    println!("{}", s);
    let mut pattern = Pattern {
        zip_positions: vec![],
        nonzip_positions: vec![],
        siteswap: vec![],
    };
    let mut pairs = SSParser::parse(Rule::notation, s).unwrap_or_else(|e| panic!("{}", e));
    let top = pairs.next().unwrap();
    assert!(pairs.next() == None);

    if top.as_rule() == Rule::shortnotation {
        let mut inner = top.into_inner();
        println!("Short");
        pattern.zip_positions = parse_zip_positions(&mut inner);

        // Next up is an optional position.
        let next = inner.peek().unwrap();
        if next.as_rule() == Rule::position {
            inner.next();
            pattern.nonzip_positions = vec![parse_position(next.into_inner().next().unwrap())];
        } else {
            pattern.nonzip_positions = vec![(Position::BottomNatural, Position::BottomNatural)];
        }
        pattern.siteswap = match inner.next().unwrap().as_rule() {
            Rule::B => vec![2u32],
            Rule::C => vec![3u32],
            Rule::F => vec![4u32],
            _ => unreachable!(),
        };
    } else if top.as_rule() == Rule::fullnotation {
        let mut inner = top.into_inner();
        pattern.zip_positions = parse_zip_positions(&mut inner);
        parse_positioned_digits(&mut pattern, &mut inner);
    }

    return pattern;
}