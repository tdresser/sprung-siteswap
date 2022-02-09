use num::integer::lcm;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::lib::data::Position;
use crate::lib::{data::Pattern, validate_siteswap::validate_siteswap};

use super::data::{Positions, DEFAULT_POSITION};

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

fn map_advanced_position(pair: Pair<Rule>) -> Position {
    return match pair.as_rule() {
        Rule::bottom_natural => Position::BottomNatural,
        Rule::bottom_opposite => Position::BottomOpposite,
        Rule::top_natural => Position::TopNatural,
        Rule::top_opposite => Position::TopOpposite,
        _ => unreachable!(),
    };
}

fn parse_position(pairs: &mut Pairs<Rule>, error: &mut Option<String>) -> (Position, Position) {
    let position = pairs.next();
    match position {
        None => unreachable!(),
        Some(pos) => match pos.as_rule() {
            Rule::crossed => return (Position::BottomOpposite, Position::BottomOpposite),
            Rule::inverted => return (Position::TopNatural, Position::TopNatural),
            Rule::crossed_inverted => return (Position::TopOpposite, Position::TopOpposite),
            Rule::natural => return (Position::BottomNatural, Position::BottomNatural),
            Rule::position_pair => {
                let mut positions = pos.into_inner();
                let p1 = positions.next().unwrap();
                let p2 = positions.next().unwrap();
                assert!(positions.next() == None);
                return (map_advanced_position(p1), map_advanced_position(p2));
            }
            _ => {
                *error = Some(format!("Invalid position {}", pairs));
                return DEFAULT_POSITION;
            }
        },
    };
}

fn parse_ambiguous_position(
    inner: &mut Pairs<Rule>,
    zip_positions: &mut Vec<(Position, Position)>,
    arc_positions: &mut Vec<(Position, Position)>,
    error: &mut Option<String>,
) {
    match inner.next().unwrap().as_rule() {
        Rule::inverted => zip_positions.push((Position::TopNatural, Position::TopNatural)),
        Rule::crossed => arc_positions.push((Position::BottomOpposite, Position::BottomOpposite)),
        Rule::crossed_inverted => {
            zip_positions.push((Position::TopNatural, Position::TopNatural));
            arc_positions.push((Position::BottomOpposite, Position::BottomOpposite));
        }
        _ => *error = Some("Failure to parse ambiguous position".to_string()),
    }
}

fn parse_digit(token: Pair<Rule>, siteswap: &mut Vec<u32>, _error: &mut Option<String>) {
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
        Rule::zip_position => zip_positions.push(parse_position(&mut token.into_inner(), error)),
        Rule::arc_position => arc_positions.push(parse_position(&mut token.into_inner(), error)),
        Rule::ambiguous_arc | Rule::ambiguous_zip | Rule::ambiguous_arc_and_zip => {
            parse_ambiguous_position(&mut token.into_inner(), zip_positions, arc_positions, error)
        }
        Rule::infix_arc_position => {
            parse_state.current_infix_modifier = parse_position(&mut token.into_inner(), error);
        }
        Rule::push_infix => arc_positions.push(parse_state.current_infix_modifier.clone()),
        Rule::digit => parse_digit(token, siteswap, error),
        Rule::B => siteswap.push(2u32),
        Rule::C => siteswap.push(3u32),
        Rule::F => siteswap.push(4u32),
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

fn error_pattern(message: String) -> Pattern {
    Pattern {
        zip_positions: vec![],
        arc_positions: vec![],
        siteswap: vec![],
        error: Some(message),
    }
}

// Collapses repetition. e.g., cici -> ci.
fn collapse<T: PartialEq + Clone + std::fmt::Debug>(ar: &Vec<T>) -> Vec<T> {
    let len = ar.len();
    for n in (1..len + 1).rev() {
        println!("N {}", n);
        // Can we divide |ar| into chunks of length n?
        if ar.len() % n != 0 {
            continue;
        }
        // Are all chunks equal?
        let mut chunks = ar.chunks_exact(len / n);
        let first = chunks.nth(0).unwrap();
        if chunks.all(|x| x == first) {
            // All chunks are equal, we're done.
            return first.to_vec();
        }
    }
    println!("{:?}", ar);
    unreachable!();
}

impl Pattern {
    fn normalized(mut self) -> Pattern {
        if self.zip_positions.len() == 0 {
            self.zip_positions.push(DEFAULT_POSITION);
        }
        if self.arc_positions.len() == 0 {
            self.arc_positions.push(DEFAULT_POSITION);
        }
        self.arc_positions = collapse(&self.arc_positions);
        self.zip_positions = collapse(&self.zip_positions);
        self.siteswap = collapse(&self.siteswap);

        let len = lcm(
            lcm(self.arc_positions.len(), self.zip_positions.len()),
            self.siteswap.len(),
        );

        // Figure out what rotation of the siteswap maximizes it's value when interpreted as an integer.
        // e.g., 315 -> 531.
        let mut max = 0u32;
        let mut max_offset = 0usize;
        for i in 0..len {
            let combined_value = self.siteswap.iter().fold(0, |r, digit| r * 10 + digit);
            if combined_value > max {
                max = combined_value;
                max_offset = i;
            }
        }

        self.arc_positions.rotate_left(max_offset);
        self.zip_positions.rotate_left(max_offset);
        self.siteswap.rotate_left(max_offset);

        return self;
    }

    pub(super) fn parse(s: &str) -> Pattern {
        println!("{}", s);
        let mut parse_state = ParseState {
            current_infix_modifier: DEFAULT_POSITION,
        };
        let mut zip_positions = vec![];
        let mut arc_positions = vec![];
        let mut siteswap = vec![];
        let mut error: Option<String> = None;

        let parse_result = SSParser::parse(Rule::notation, s);
        if let Err(e) = parse_result {
            println!("ERROR! {}", e.to_string());
            return error_pattern(e.to_string());
        }
        let mut pairs = parse_result.unwrap();
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

        match validate_siteswap(&siteswap) {
            Err(message) => {
                return error_pattern(message);
            }
            Ok(()) => {
                let mut pattern = Pattern {
                    zip_positions,
                    arc_positions,
                    siteswap,
                    error: None,
                };
                pattern = pattern.normalized();
                return pattern;
            }
        }
    }
}
