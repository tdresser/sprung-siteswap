use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "normalize.pest"]
struct SSParser;

#[derive(Debug)]
struct Pattern {
    zip_positions: Vec<(Position, Position)>,
    nonzip_positions: Vec<(Position, Position)>,
    siteswap: Vec<u32>,
}

fn get_zip_position_str((a, b): &(Position, Position)) -> String {
    return match (a, b) {
        (Position::BottomNatural, &Position::BottomNatural) => "n",
        _ => unreachable!(),
    }
    .to_string();
}

impl Pattern {
    fn get_canonical_form(self) -> String {
        let mut result = "".to_string();
        let current_position = "n".to_string();
        for zip_position in &self.zip_positions {
            result = format!("{}{}z", result, get_zip_position_str(zip_position));
        }
        return result;
    }
}

#[derive(Debug, PartialEq)]
enum Position {
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

fn parse(s: &str) -> Pattern {
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
        }
        pattern.siteswap = match inner.next().unwrap().as_rule() {
            Rule::B => vec![2u32],
            Rule::C => vec![3u32],
            Rule::F => vec![4u32],
            _ => unreachable!(),
        };
        print_pairs(&mut inner);
    } else if top.as_rule() == Rule::fullnotation {
        let mut inner = top.into_inner();
        println!("Full");
        pattern.zip_positions = parse_zip_positions(&mut inner);
        parse_positioned_digits(&mut pattern, &mut inner);
        print_pairs(&mut inner);
    }

    return pattern;
}

fn main() {
    let mut pattern = parse("czS312");
    println!("{:?}\n", pattern);

    pattern = parse("cB");
    println!("{:?}\n", pattern);

    pattern = parse("izcB");
    println!("{:?}\n", pattern);

    pattern = parse("czSc3ci1n2");
    println!("{:?}\n", pattern);

    pattern = parse("izczSc3ci1n2");
    println!("{:?}\n", pattern);

    pattern = parse("bbzSTb3c1ic2");
    println!("{:?}\n", pattern);

    pattern = parse("TbzBbB");
    println!("{:?}\n", pattern);
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use crate::Position;

    #[test]
    fn box_base() {
        let pattern = parse("cB");
        assert_eq!(pattern.siteswap, vec![2]);
        assert_eq!(
            pattern.nonzip_positions,
            vec![(Position::BottomOpposite, Position::BottomOpposite)]
        );
    }

    #[test]
    fn cascade_base() {
        let pattern = parse("iC");
        assert_eq!(pattern.siteswap, vec![3]);
        assert_eq!(
            pattern.nonzip_positions,
            vec![(Position::TopNatural, Position::TopNatural)]
        );
    }

    #[test]
    fn fountain_base() {
        let pattern = parse("ciF");
        assert_eq!(pattern.siteswap, vec![4]);
        assert_eq!(
            pattern.nonzip_positions,
            vec![(Position::TopOpposite, Position::TopOpposite)]
        );
    }

    #[test]
    fn sprung_base() {
        let pattern = parse("icS312");
        assert_eq!(pattern.siteswap, vec![3, 1, 2]);
        assert_eq!(
            pattern.nonzip_positions,
            vec![(Position::TopOpposite, Position::TopOpposite)]
        );
    }
}
