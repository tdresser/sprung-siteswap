use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "normalize.pest"]
struct SSParser;

#[derive(Debug)]
struct Pattern {
    nonzip_positions: Vec<String>,
    siteswap: Vec<u32>,
}

fn normalize(pairs: Pairs<Rule>, pattern: &mut Pattern) {
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
}

fn parse(s: &str) -> Pattern {
    let mut pattern = Pattern {
        nonzip_positions: vec![],
        siteswap: vec![],
    };
    let pairs = SSParser::parse(Rule::notation, s).unwrap_or_else(|e| panic!("{}", e));
    normalize(pairs, &mut pattern);
    return pattern;
}

fn main() {
    let mut pattern = parse("cS312");
    println!("{:?}\n", pattern);
    pattern = parse("cB");
    println!("{:?}\n", pattern);
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn box_base() {
        let pattern = parse("cB");
        assert_eq!(pattern.siteswap, vec![2]);
        assert_eq!(pattern.nonzip_positions, vec!["c"]);
    }

    #[test]
    fn cascade_base() {
        let pattern = parse("iC");
        assert_eq!(pattern.siteswap, vec![3]);
        assert_eq!(pattern.nonzip_positions, vec!["i"]);
    }

    #[test]
    fn fountain_base() {
        let pattern = parse("ciF");
        assert_eq!(pattern.siteswap, vec![4]);
        assert_eq!(pattern.nonzip_positions, vec!["ci"]);
    }

    #[test]
    fn sprung_base() {
        let pattern = parse("icS312");
        assert_eq!(pattern.siteswap, vec![3, 1, 2]);
        assert_eq!(pattern.nonzip_positions, vec!["ic"]);
    }
}
