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
            Rule::s => print!("{}", pair.as_str()),
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
            Rule::position => println!("position: {}", pair.as_str()),
            Rule::digit => unreachable!(),
            Rule::ss => unreachable!(),
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
