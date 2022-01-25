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

fn parse_position(pairs: &mut Pairs<Rule>, into: &mut Positions, error: &mut Option<String>) {
    let position = pairs.next();
    match position {
        None => unreachable!(),
        Some(pos) => match pos.as_rule() {
            Rule::crossed => into.push((Position::BottomOpposite, Position::BottomOpposite)),
            Rule::inverted => into.push((Position::TopNatural, Position::TopNatural)),
            Rule::crossed_inverted => into.push((Position::TopOpposite, Position::TopOpposite)),
            Rule::natural => into.push((Position::BottomNatural, Position::BottomNatural)),
            Rule::position_pair => {
                let mut positions = pos.into_inner();
                let p1 = positions.next().unwrap();
                let p2 = positions.next().unwrap();
                assert!(positions.next() == None);
                into.push((map_advanced_position(p1), map_advanced_position(p2)));
            }
            _ => {
                *error = Some(format!("Invalid position {}", pairs));
                unreachable!("Invalid position {} {}", pairs, pos);
            }
        },
    };
}

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

fn parse_digit(token: Pair<Rule>, siteswap: &mut Vec<u32>, error: &mut Option<String>) {
    let s = token.as_str();
    assert!(s.len() == 1, "{}", s);
    let c = s.chars().nth(0).unwrap();
    siteswap.push(c.to_digit(10).unwrap());
}

fn parse_token(
    token: Pair<Rule>,
    parse_state: &mut ParseState,
    zip_positions: &mut Positions,
    arc_positions: &mut Positions,
    siteswap: &mut Vec<u32>,
    error: &mut Option<String>,
) {
    let str = token.as_str();
    match token.as_rule() {
        Rule::zip_position => parse_position(&mut token.into_inner(), zip_positions, error),
        Rule::arc_position => parse_position(&mut token.into_inner(), arc_positions, error),
        Rule::ambiguous_position => println!("AB"),
        Rule::digit => parse_digit(token, siteswap, error),
        Rule::EOI => {
            return;
        }
        _ => {
            *error = Some(format!("Invalid token: "));
        }
    };
    match error {
        Some(message) => {
            *error = Some(format!("{} {}", message, str));
        }
        None => (),
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
        let mut error: Option<String> = None;

        let mut pairs = SSParser::parse(Rule::notation, s).unwrap_or_else(|e| panic!("{}", e));
        let inner = pairs.next().unwrap().into_inner();
        println!("{}", pairs);
        for token in inner {
            parse_token(
                token,
                &mut parse_state,
                &mut zip_positions,
                &mut arc_positions,
                &mut siteswap,
                &mut error,
            );

            match error {
                None => (),
                Some(ref message) => {
                    println!("ERROR! {}", message);
                    break;
                }
            }
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
