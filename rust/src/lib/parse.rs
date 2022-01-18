use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::lib::data::Pattern;
use crate::lib::data::Position;

#[derive(Parser)]
#[grammar = "normalize.pest"]
struct SSParser;

#[allow(dead_code)]
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

fn parse_positioned_digits(
    positions: &mut Vec<(Position, Position)>,
    siteswap: &mut Vec<u32>,
    pairs: &mut Pairs<Rule>,
) {
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

impl Pattern {
    pub(super) fn parse(s: &str) -> Pattern {
        println!("{}", s);
        let mut zip_positions = vec![];
        let mut nonzip_positions = vec![];
        let mut siteswap = vec![];

        let mut pairs = SSParser::parse(Rule::notation, s).unwrap_or_else(|e| panic!("{}", e));
        let top = pairs.next().unwrap();
        assert!(pairs.next() == None);

        if top.as_rule() == Rule::shortnotation {
            let mut inner = top.into_inner();
            println!("Short");
            zip_positions = parse_zip_positions(&mut inner);

            // Next up is an optional position.
            let next = inner.peek().unwrap();
            if next.as_rule() == Rule::position {
                inner.next();
                nonzip_positions = vec![parse_position(next.into_inner().next().unwrap())];
            } else {
                nonzip_positions = vec![(Position::BottomNatural, Position::BottomNatural)];
            }
            siteswap = match inner.next().unwrap().as_rule() {
                Rule::B => vec![2u32],
                Rule::C => vec![3u32],
                Rule::F => vec![4u32],
                _ => unreachable!(),
            };
        } else if top.as_rule() == Rule::fullnotation {
            let mut inner = top.into_inner();
            zip_positions = parse_zip_positions(&mut inner);
            parse_positioned_digits(&mut nonzip_positions, &mut siteswap, &mut inner);
        }

        return Pattern {
            zip_positions,
            nonzip_positions,
            siteswap,
            error: None,
        };
    }
}
