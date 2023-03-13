extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[grammar = "DeRust.pest"]
pub struct DeRustParser;

fn main() -> std::io::Result<()> {
    let rule_test = "0001_def_fn__function_call";

    let input = fs::read_to_string(&(String::from("./test/") + rule_test + ".drs")).expect("cannot read file");
    let pair = DeRustParser::parse(Rule::file, &input).unwrap().next().unwrap();
    println!("{:#?}", pair.clone()); //test
    let s: String = output(pair);

    let rust_file_path = String::from("./test/") + rule_test + ".drs.rs";
    println!("\nRAW RUST FILE:\n    {:?}", s);
    fs::write(&rust_file_path, &s)?;
    Command::new("rustfmt").arg(&rust_file_path).status().expect("");
    println!("\nRUST FILE:\n");
    Command::new("cat").arg(&rust_file_path).status().expect("");

    println!("\nDONE: {}.drs", rule_test);
    // test_except(rule_test)
    Ok(())
}

fn test_except(present: &str) -> std::io::Result<()> {
    let mut mark = false;
    let mut i: i32 = 1;
    for entry in fs::read_dir("./test")? {
        let file = entry.unwrap().path();
        let file = file.to_str().unwrap();
        if file.ends_with(".drs") && !file.ends_with(&(String::from(present) + ".drs")) {
            let drs_content = fs::read_to_string(file).expect("err when read {file}");
            let pair = DeRustParser::parse(Rule::file, &drs_content).unwrap().next().unwrap();
            let s: String = output(pair);

            let temp_file = String::from("./test/temp/") + &(i.to_string());

            fs::write(&temp_file, &s)?;
            Command::new("rustfmt").arg(&temp_file).status().expect("");

            let rs_content = fs::read_to_string(String::from(file) + ".rs").expect("err when read {file}");
            let now_content = fs::read_to_string(String::from(temp_file)).expect("err when read {temp_file}");

            if rs_content != now_content {
                let (_, temp_s) = file.split_at(7);
                if mark == false {
                    println!("\nChanged file:");
                }
                println!("    {temp_s}.rs");
                let now_rust_file_path = String::from(file) + ".rs.now";
                fs::write(&now_rust_file_path, &now_content)?;
                mark = true;
            }
            i = i + 1;
        }
    }
    Ok(())
}

fn output(pair: Pair<Rule>) -> String {
    let mut s = String::new();
    let rule = pair.as_rule();
    match rule {
        | Rule::file => {
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            return s;
        },
        | Rule::function_call_expr | Rule::function_call_statement => {
            let mut identifier = String::new();
            let mut parameters = String::new();
            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    | Rule::identifier | Rule::identifier_atomic => {
                        identifier.push_str(&output(subpair));
                        identifier.push_str("_");
                    },
                    | Rule::expression => {
                        parameters.push_str(&output(subpair));
                        parameters.push_str(",");
                    },
                    | _ => {},
                }
            }
            identifier.pop();
            s.push_str(&identifier);
            s.push_str("(");
            s.push_str(&parameters);
            s.push_str(")");
            if rule == Rule::function_call_statement {
                s.push_str(";");
            }
            return s;
        },
        | Rule::number_literal => {
            return pair.as_str().replace(" ", "_");
        },
        | Rule::identifier => {
            return pair.as_str().replace(" ", "_");
        },
        | Rule::def_fn_head => {
            let mut identifier = String::new();
            let mut result = String::new();
            let mut parameters = String::new();
            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    | Rule::identifier | Rule::identifier_atomic => {
                        identifier.push_str(&output(subpair));
                        identifier.push_str("_");
                    },
                    | Rule::type_expr => {
                        parameters.push_str(&output(subpair));
                        parameters.push_str(", ");
                    },
                    | Rule::type_name => {
                        result.push_str("->");
                        result.push_str(&output(subpair));
                    },
                    | _ => {},
                }
            }
            identifier.pop();
            s.push_str(&identifier);
            s.push_str("(");
            s.push_str(&parameters);
            s.push_str(")");
            s.push_str(&result);
            return s;
        },
        | Rule::def_fn_body => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str("}");
            return s;
        },
        | Rule::tuple_expr => {
            s.push_str("(");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(",");
            }
            s.push_str(")");
            return s;
        },
        // TODO: test
        | Rule::triple_quote_string => {
            s.push_str("\"");
            s.push_str(&output(pair.into_inner().next().unwrap()));
            s.pop();
            s.push_str("\"");
            return s;
        },

        // 具有固定子规则的规则, 用 inner_rules.next().unwrap() 解析.
        | Rule::def_fn => {
            let mut inner_rules = pair.into_inner();
            s.push_str("fn ");
            s.push_str(&output(inner_rules.next().unwrap()));
            s.push_str(&output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::type_expr => {
            let mut inner_rules = pair.into_inner();
            s.push_str(&output(inner_rules.next().unwrap()));
            s.push_str(": ");
            s.push_str(&output(inner_rules.next().unwrap()));
            return s;
        },

        // enmu类规则 直接跳到 子规则
        | Rule::type_name | Rule::string_literal | Rule::expression | Rule::statement | Rule::expr_literal => {
            return output(pair.into_inner().next().unwrap());
        },

        // 直接返回原值的规则
        | Rule::quote_string
        | Rule::raw_string
        | Rule::inner_string
        | Rule::bool_literal
        | Rule::identifier_atomic
        | Rule::EOI => {
            return String::from(pair.as_str());
        },

        // TODO: 这里的规则都是待处理的规则, 穷尽规则之后必须把这条直接删除.
        | _ => {
            println!("skip rule: {:?}\n{:?}", pair.as_rule(), pair.as_str());
            return String::from(pair.as_str());
        },
    }
}
