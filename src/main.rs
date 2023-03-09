extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "DeRust.pest"]
pub struct DeRustParser;

fn main() {
    let input = fs::read_to_string("./test/def_fn/def_fn.drs").expect("cannot read file");

    let pairs = DeRustParser::parse(Rule::file, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    println!("{:#?}", pairs);

    let mut output: String = String::new();
    for pair in pairs {
        match pair.as_rule() {
            | Rule::def_fn => {
                output = output + &output_def_fn(pair);
            },
            | _ => {},
        }
    }

    println!("{:#?}", output);
}

fn output_def_fn(pair: Pair<Rule>) -> String {
    let mut inner_rules = pair.into_inner();
    let pair_head = inner_rules.next().unwrap();
    let pair_body = inner_rules.next().unwrap();

    let mut identifier = String::new();
    let mut result = String::new();
    let mut parameters = String::new();
    parameters.push_str("(");

    for subpair in pair_head.into_inner() {
        // println!("{:#?}", subpair.as_rule());
        match subpair.as_rule() {
            | Rule::identifier => {
                identifier.push_str(&output_identifier(subpair));
            },
            | Rule::identifier_atomic => {
                identifier.push_str("_");
                identifier.push_str(subpair.as_str());
            },
            | Rule::def_fn_head_result => {
                result.push_str(" ");
                result.push_str(subpair.as_str());
            },
            | Rule::variable_def_head => {
                parameters.push_str(subpair.as_str());
                parameters.push_str(", ");
            },
            | _ => {},
        }
    }

    parameters.push_str(")");

    let mut output: String = String::new();
    output.push_str("fn ");
    output.push_str(&identifier);
    output.push_str(&parameters);
    output.push_str(&result);
    output.push_str(&output_body(pair_body));

    return output;
}

fn output_identifier(pair: Pair<Rule>) -> String {
    let mut output = String::new();
    for subpair in pair.into_inner() {
        match subpair.as_rule() {
            | Rule::identifier_atomic => {
                output.push_str(subpair.as_str());
                output.push_str("_");
            },
            | _ => {},
        }
    }
    output.pop();
    return output;
}

fn output_body(pair: Pair<Rule>) -> String {
    return String::from("{}");
}
