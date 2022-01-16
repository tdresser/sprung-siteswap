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

/*fn normalize(pairs: Pairs<Rule>, pattern: &mut Pattern) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::notation => normalize(pair.into_inner(), pattern),
            Rule::base => {
                pattern.siteswap = match pair.as_str().chars().nth(0).unwrap() {
                    'B' => vec![2u32],
                    'C' => vec![3u32],
                    'F' => vec![4u32],
                    'S' => {
                        let mut chars = pair.as_str().chars();
                        // Skip over the S.
                        chars.next();
                        println!("CHARS: {:?}", chars);
                        let digits = chars.map(|x: char| x.to_digit(16).unwrap());
                        digits.collect::<Vec<_>>()
                    }
                    _ => unreachable!(),
                };
            }
            Rule::position => pattern.nonzip_positions = vec![pair.as_str().to_string()],
            Rule::s | Rule::ss | Rule::digit => unreachable!(),
        };
    }
}*/

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
        // TODO, the hard case.
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

/*fn print_pair(pair: Pair<Rule>) {
    println!("{}", pair.as_str());
    for p in pair.into_inner() {
        println!("{}", p);
    }
}*/

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

    //let top = &pairs[0];
    //let top = pairs[0].as_rule();

    if top.as_rule() == Rule::shortnotation {
        let mut inner = top.into_inner();
        println!("Short");
        pattern.zip_positions = parse_zip_positions(&mut inner);
        print_pairs(&mut inner);

        //pattern.nonzip_positions = parse_nonzip_positions(&mut inner);
    } else if top.as_rule() == Rule::fullnotation {
        let mut inner = top.into_inner();
        println!("Full");
        pattern.zip_positions = parse_zip_positions(&mut inner);
        print_pairs(&mut inner);
    }

    /*for pair in pairs {
        match pair.as_rule() {
            Rule::notation => normalize(pair.into_inner(), pattern),
            Rule::base => {
                pattern.siteswap = match pair.as_str().chars().nth(0).unwrap() {
                    'B' => vec![2u32],
                    'C' => vec![3u32],
                    'F' => vec![4u32],
                    'S' => {
                        let mut chars = pair.as_str().chars();
                        // Skip over the S.
                        chars.next();
                        println!("CHARS: {:?}", chars);
                        let digits = chars.map(|x: char| x.to_digit(16).unwrap());
                        digits.collect::<Vec<_>>()
                    }
                    _ => unreachable!(),
                };
            }
            Rule::position => pattern.nonzip_positions = vec![pair.as_str().to_string()],
            Rule::s | Rule::ss | Rule::digit => unreachable!(),
        };
    }*/

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
