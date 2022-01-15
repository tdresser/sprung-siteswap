use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct SSParser;

fn print_pairs(pairs: Pairs<Rule>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::notation => print_pairs(pair.into_inner()),
            Rule::ss => print_pairs(pair.into_inner()),
            Rule::digit => print!("<{}>", pair.as_str()),
            Rule::s => print!("{}", pair.as_str()),
        };
    }
}

fn main() {
    let pairs = SSParser::parse(Rule::notation, "S312").unwrap_or_else(|e| panic!("{}", e));

    print_pairs(pairs);
}
