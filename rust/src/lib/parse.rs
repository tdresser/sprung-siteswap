use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::lib::data::Position;
use crate::lib::{data::Pattern, validate_siteswap::validate_siteswap};

use super::data::Positions;

#[derive(Parser)]
#[grammar = "normalize.pest"]
struct SSParser;

struct ParseState {
    pub current_infix_modifier: (Position, Position),
}

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
/*
fn parse_positioned_digits(
    positions: &mut Positions,
    siteswap: &mut Vec<u32>,
    pairs: &mut Pairs<Rule>,
) {
    loop {
        match pairs.peek() {
            None => break,
            Some(pair) => {
                match pair.as_rule() {
                    Rule::position => {
                        pairs.next();
                        positions.push(parse_position(pair));
                    }
                    _ => positions.push((Position::BottomNatural, Position::BottomNatural)),
                }
                let digit_or_cf = pairs.next().unwrap();
                match digit_or_cf.as_rule() {
                    Rule::digit => {
                        let s = digit_or_cf.as_str();
                        assert!(s.len() == 1);
                        let c = s.chars().nth(0).unwrap();
                        siteswap.push(c.to_digit(10).unwrap());
                    }
                    Rule::C => siteswap.push(3u32),
                    Rule::F => siteswap.push(4u32),
                    _ => {
                        println!("{}", digit_or_cf);
                        unreachable!()
                    }
                }
            }
        }
    }
    assert!(pairs.next() == None);
}*/

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

/*fn parse_zip_positions(pairs: &mut Pairs<Rule>) -> Positions {
    let mut positions: Positions = vec![];
    println!("ZP: {}", pairs);
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
}*/

/*fn parse_ambiguous_position(
    pairs: &mut Pairs<Rule>,
    zip_positions: &mut Positions,
    arc_postions: &mut Positions,
) {
    println!("{}", pairs);
    assert!(pairs.peek().unwrap().as_rule() == Rule::ambiguous_position);
    let inner = pairs.next().unwrap();
    match inner.as_rule() {
        Rule::inverted => ,
        Rule::crossed => ,
        Rule::crossed_inverted,
    }
}*/

fn parse_token(
    pair: &Pair<Rule>,
    parse_state: &mut ParseState,
    zip_positions: &[(Position, Position)],
    arc_positions: &[(Position, Position)],
    siteswap: &[u32],
) {
    match pair.as_rule() {
        Rule::zip_position => println!("ZP"),
        Rule::arc_position => println!("AP"),
        Rule::ambiguous_position => println!(),
    }
}

impl Pattern {
    pub(super) fn parse(s: &str) -> Pattern {
        println!("{}", s);
        let mut parse_state = ParseState {
            current_infix_modifier: (Position::BottomNatural, Position::BottomNatural),
        };
        let mut zip_positions = vec![];
        let mut arc_positions = vec![];
        let mut siteswap = vec![];

        let mut pairs = SSParser::parse(Rule::notation, s).unwrap_or_else(|e| panic!("{}", e));
        println!("{}", pairs);
        for token in pairs.next().unwrap().into_inner() {
            parse_token(
                &token,
                &mut parse_state,
                &mut zip_positions,
                &mut arc_positions,
                &mut siteswap,
            );
        }
        /*let top = pairs.next().unwrap();
        assert!(pairs.next().unwrap().as_rule() == Rule::EOI);

        if top.as_rule() == Rule::boxnotation {
            let mut inner = top.into_inner();
            println!("Box");
            zip_positions = parse_zip_positions(&mut inner);

            // Next up is an optional position.
            let next = inner.peek().unwrap();
            if next.as_rule() == Rule::position {
                inner.next();
                arc_positions = vec![parse_position(next.into_inner().next().unwrap())];
            } else {
                arc_positions = vec![(Position::BottomNatural, Position::BottomNatural)];
            }
            siteswap = vec![2u32];
        } else if top.as_rule() == Rule::fullnotation {
            let mut inner = top.into_inner();
            zip_positions = parse_zip_positions(&mut inner);
            parse_ambiguous_position(&mut inner, &mut zip_positions, &mut arc_positions);
            parse_positioned_digits(&mut arc_positions, &mut siteswap, &mut inner);
        }

        match validate_siteswap(&siteswap) {
            Err(message) => {
                return {
                    Pattern {
                        zip_positions: vec![],
                        arc_positions: vec![],
                        siteswap: vec![],
                        error: Some(message),
                    }
                }
            }
            Ok(()) => {*/
        return Pattern {
            zip_positions,
            arc_positions,
            siteswap,
            error: None,
            /*};
            }*/
        };
    }
}
