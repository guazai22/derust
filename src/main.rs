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

fn main() {
    let rule_testing =
        //# 规则条目:
        // "0001_def_fn__function_call"
        // "0002_array_expr"
        // "0003_number_literal"
        // "0004_lambda_expr"
        // "0005_match_expr"
        // "0006_loop_expr"
        // "0007_if_expr"
        // "0008_block_return_expr"
        "0009_def_fn"
    ;

    let input = fs::read_to_string(&(String::from("./test/") + rule_testing + ".drs"))
        .expect("cannot read file");
    let pair = DeRustParser::parse(Rule::file, &input).unwrap().next().unwrap();
    // println!("{:#?}", pair); // test
    let inital_rs_file: String = output(pair);

    let rust_file_path = String::from("./test/") + rule_testing + ".drs.rs";
    println!("\nRAW RUST FILE:\n    {:?}", inital_rs_file);
    fs::write(&rust_file_path, &inital_rs_file).unwrap();
    Command::new("rustfmt").arg(&rust_file_path).status().expect("");
    println!("\nFORMATED RUST FILE:");
    Command::new("cat").arg(&rust_file_path).status().expect("");
    println!("\nDONE: {}.drs", rule_testing);

    // test_if_other_rules_still_good(rule_testing);
    // test_if_err_is_err();
}

fn test_if_err_is_err() {
    let mut mark = false;
    for entry in fs::read_dir("./test").unwrap() {
        let path = entry.unwrap().path();
        let path = path.to_str().unwrap();
        if path.ends_with(".err") {
            let (_, err_file) = path.split_at(7);
            let mut i = 0;

            for line in fs::read_to_string(path).unwrap().lines() {
                i = i + 1;
                if let Ok(_) = DeRustParser::parse(Rule::file, line) {
                    if mark == false {
                        println!("\nGOOD LINES IN ERR FILE:");
                        mark = true;
                    }
                    println!("   {}:   line {}", err_file, i);
                }
            }
        }
    }
}

fn test_if_other_rules_still_good(present: &str) {
    let mut has_print_title = false;
    let mut i: i32 = 1;

    for entry in fs::read_dir("./test").unwrap() {
        let file = entry.unwrap().path();
        let file = file.to_str().unwrap();

        if file.ends_with(".drs") && !file.ends_with(&(String::from(present) + ".drs")) {
            let drs_content = fs::read_to_string(file).expect("err when read {file}");
            let pair = DeRustParser::parse(Rule::file, &drs_content).unwrap().next().unwrap();
            let s: String = output(pair);

            let temp_file = String::from("./test/temp/") + &(i.to_string());
            fs::write(&temp_file, &s).unwrap();
            Command::new("rustfmt").arg(&temp_file).status().expect("");

            let rs_content =
                fs::read_to_string(String::from(file) + ".rs").expect("err when read {file}");
            let now_content =
                fs::read_to_string(String::from(temp_file)).expect("err when read {temp_file}");
            assert_eq!(1, 1);

            if rs_content != now_content {
                let (_, temp_s) = file.split_at(7);
                if has_print_title == false {
                    println!("\nthese files are parsed diffriently from last time:");
                }
                println!("    {temp_s}.rs");
                let now_rust_file_path = String::from(file) + ".rs.now";
                fs::write(&now_rust_file_path, &now_content).unwrap();
                has_print_title = true;
            }
            i = i + 1;
        }
    }
}

fn output(pair: Pair<Rule>) -> String {
    let mut s = String::new();
    let rule = pair.as_rule();

    // 具有不固定数量的不同各类子规则的规则, 用
    // for subpair in pair.into_inner() {
    //     match subpair.as_rule() {
    // 解析
    match rule {
        | Rule::function_call_expr => {
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
            return s;
        },
        | Rule::fn_def_identifier_1 | Rule::fn_def_identifier_2plus => {
            let mut identifier = String::new();
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
                    | _ => {},
                }
            }
            identifier.pop();
            s = format!("{}({})", identifier, parameters);
            return s;
        },

        // 具有不固定数量同样子规则的规则, 用 for subpair in pair.into_inner() 解析
        | Rule::array_some => {
            s.push_str("[");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(",");
            }
            s.push_str("]");
            return s;
        },
        | Rule::block_expr => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str("}");
            return s;
        },
        | Rule::block_statement => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str("}");
            return s;
        },
        | Rule::def_fn => {
            s.push_str("fn ");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            return s;
        },
        | Rule::def_fn_main => {
            s.push_str("fn main()");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            return s;
        },
        | Rule::file => {
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            return s;
        },
        | Rule::identifier => {
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str("_");
            }
            s.pop();
            return s;
        },
        // 这里需要对 if_expr_derust 转变成 rust 语句进行变形.
        | Rule::if_expr_rust | Rule::if_expr_derust => {
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            return s;
        },
        | Rule::lambda_head => {
            s.push_str("|");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(", ");
            }
            s.push_str("|");
            return s;
        },
        | Rule::let_statement => {
            s.push_str("let ");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str(";");
            return s;
        },
        | Rule::match_branches_expr => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(", ");
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

        // 具有固定数量子规则的规则, 用 inner_rules.next().unwrap() 解析.
        | Rule::array_repeat => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "[{}; {}]",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::assignment => {
            let mut inner_rules = pair.into_inner();
            s = format!("= {}", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::else_branch => {
            let mut inner_rules = pair.into_inner();
            s = format!("else {}", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::else_if_branch => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "else if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::fn_type => {
            let mut inner_rules = pair.into_inner();
            s = format!("-> {}", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::function_call_statement => {
            let mut inner_rules = pair.into_inner();
            s = format!("{};", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::if_branch => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        // | Rule::if_expr_when => {
        //     let mut inner_rules = pair.into_inner();
        //     let b = inner_rules.next().unwrap();
        //     let a = inner_rules.next().unwrap();
        //     let else_c = inner_rules.next().unwrap();
        //     s = format!("if {} {} {}", a, b, else_c);
        //     return s;
        // },
        | Rule::lambda_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "{} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::loop_times_expr => {
            let mut inner_rules = pair.into_inner();
            let mut inner_rule = inner_rules.next().unwrap();
            let mut loop_mark = String::new();
            if inner_rule.as_rule() == Rule::loop_mark {
                loop_mark = output(inner_rule);
                inner_rule = inner_rules.next().unwrap();
            }
            if inner_rule.as_rule() == Rule::expression {
                s.push_str(" { let mut i = 0;");
                s.push_str(&loop_mark);
                s.push_str(" while ( i < ( ");
                s.push_str(&output(inner_rule));
                s.push_str(")) { i = i + 1;");
                for subpair in inner_rules.next().unwrap().into_inner() {
                    s.push_str(&output(subpair));
                }
                s.push_str("}}");
            } else {
                s = format!("{} loop {}", &loop_mark, output(inner_rule));
            }
            return s;
        },
        | Rule::loop_for_expr => {
            let mut inner_rules = pair.into_inner();
            let inner_rule = inner_rules.next().unwrap();
            if inner_rule.as_rule() == Rule::loop_mark {
                s = format!(
                    "{} for ({}) in ({}) {}",
                    output(inner_rule),
                    output(inner_rules.next().unwrap()),
                    output(inner_rules.next().unwrap()),
                    output(inner_rules.next().unwrap()),
                );
            } else {
                s = format!(
                    "for ({}) in ({}) {}",
                    output(inner_rule),
                    output(inner_rules.next().unwrap()),
                    output(inner_rules.next().unwrap()),
                );
            }
            return s;
        },
        | Rule::loop_mark => {
            let mut inner_rules = pair.into_inner();
            s = format!("'{}:", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::loop_repeat_expr => {
            let mut inner_rules = pair.into_inner();
            let inner_rule = inner_rules.next().unwrap();
            if inner_rule.as_rule() == Rule::loop_mark {
                s = format!("{} loop {}", output(inner_rule), output(inner_rules.next().unwrap()));
            } else {
                s = format!("loop {}", output(inner_rule));
            }
            return s;
        },
        | Rule::loop_while_expr => {
            let mut inner_rules = pair.into_inner();
            let inner_rule = inner_rules.next().unwrap();
            if inner_rule.as_rule() == Rule::loop_mark {
                s = format!(
                    "{} while ({}) {}",
                    output(inner_rule),
                    output(inner_rules.next().unwrap()),
                    output(inner_rules.next().unwrap())
                );
            } else {
                s = format!(
                    "while ({}) {}",
                    output(inner_rule),
                    output(inner_rules.next().unwrap())
                );
            }
            return s;
        },
        | Rule::match_expr_derust => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "match {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::match_branch_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "{} => {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::match_branch_else_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!("_ => {}", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::block_return_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!("return {};", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::block_return_statement => {
            s.push_str("return ;");
            return s;
        },
        | Rule::sub_if_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::sub_else_if_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "else if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::sub_else_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!("else {}", output(inner_rules.next().unwrap()));
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
        | Rule::type_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "{}: {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },

        // 直接对 pair.as_str() 处理的规则
        | Rule::measure_with_number | Rule::number_literal => {
            return pair.as_str().replace(" ", "_");
        },

        //  直接返回 pair.as_str() 的规则
        | Rule::array_none
        | Rule::bool_literal
        | Rule::EOI
        | Rule::identifier_atomic
        | Rule::inner_string
        | Rule::quote_string
        | Rule::raw_string => {
            return pair.as_str().to_string();
        },

        // enmu类规则, 或者只有一条有效子规则的规则, 直接跳到 子规则
        | Rule::array_expr
        | Rule::branch_expr
        | Rule::block
        | Rule::brackt_expr
        | Rule::expression
        | Rule::fn_def_identifier
        | Rule::if_expr
        | Rule::literal_expr
        | Rule::loop_expr
        | Rule::match_expr
        | Rule::module
        | Rule::statement
        | Rule::string_literal
        | Rule::type_name => {
            return output(pair.into_inner().next().unwrap());
        },

        // TODO: 这里显示的规则都是待处理的规则, 理论上不该match _ ,
        // 以后穷尽规则之后必须把这条直接删除.
        | _ => {
            println!("skip rule: {:?}\n{:?}", pair.as_rule(), pair.as_str());
            return pair.as_str().to_string();
        },
    }
}
