extern crate pest;

use pest::iterators::Pair;

use pest::Parser;

mod b;
use b::to_rust_function;

#[derive(Parser)]
#[grammar = "DeRust.pest"]
struct DeRustParser;

pub fn derust_to_rust(input: String) -> String {
    let pair = DeRustParser::parse(Rule::file, &input).unwrap().next().unwrap();
    output(pair)
}

fn output(pair: Pair<Rule>) -> String {
    let mut s = String::new();
    let rule = pair.as_rule();

    // 子规则数量不固定, 类型也不一样, 用
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
            identifier = to_rust_function(&identifier).to_string();
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

        // 子规则数量不固定, 类型一样, 用 for subpair in pair.into_inner() 解析
        | Rule::array_some => {
            s.push_str("[");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(",");
            }
            s.push_str("]");
            return s;
        },
        | Rule::block => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str("}");
            return s;
        },
        | Rule::break_statement => {
            let mut mark = String::new();
            let mut expr = String::new();
            for subpair in pair.into_inner() {
                match subpair.as_rule() {
                    | Rule::loop_mark => {
                        mark.push_str(&output(subpair));
                        mark.pop();
                    },
                    | Rule::expression => {
                        expr.push_str(&output(subpair));
                    },
                    | _ => {},
                }
            }
            s = format!("break {} {};", mark, expr);
            return s;
        },
        | Rule::continue_statement => {
            s.push_str("continue ");
            for subpair in pair.into_inner() {
                s.push_str("'");
                s.push_str(&output(subpair.into_inner().next().unwrap()));
            }
            s.push_str(";");
            return s;
        },
        | Rule::def_fn => {
            s.push_str("fn ");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(" ");
            }
            return s;
        },
        | Rule::def_fn_main => {
            s.push_str("fn main()");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(" ");
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
        | Rule::rust_if_block | Rule::derust_if_block => {
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
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(" ");
            }
            s.push_str(";");
            return s;
        },
        | Rule::match_branches => {
            s.push_str("{");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
                s.push_str(", ");
            }
            s.push_str("}");
            return s;
        },
        | Rule::return_statement => {
            s.push_str("return ");
            for subpair in pair.into_inner() {
                s.push_str(&output(subpair));
            }
            s.push_str(";");
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

        // 子规则的数量固定, 用 inner_rules.next().unwrap() 解析.
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
        | Rule::dot_chain_expr => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "{}.{}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::dot_chain_statement => {
            let mut inner_rules = pair.into_inner();
            s = format!("{};", output(inner_rules.next().unwrap()),);
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
        | Rule::loop_times_block => {
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
        | Rule::loop_for_block => {
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
        | Rule::loop_repeat_block => {
            let mut inner_rules = pair.into_inner();
            let inner_rule = inner_rules.next().unwrap();
            if inner_rule.as_rule() == Rule::loop_mark {
                s = format!("{} loop {}", output(inner_rule), output(inner_rules.next().unwrap()));
            } else {
                s = format!("loop {}", output(inner_rule));
            }
            return s;
        },
        | Rule::loop_while_block => {
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
        | Rule::match_block => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "match {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::match_branch => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "{} => {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::match_else_branch => {
            let mut inner_rules = pair.into_inner();
            s = format!("_ => {}", output(inner_rules.next().unwrap()));
            return s;
        },
        | Rule::sub_if_block => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::sub_else_if_block => {
            let mut inner_rules = pair.into_inner();
            s = format!(
                "else if {} {}",
                output(inner_rules.next().unwrap()),
                output(inner_rules.next().unwrap())
            );
            return s;
        },
        | Rule::sub_else_block => {
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
        | Rule::KEYWORD_let
        | Rule::quote_string
        | Rule::raw_string => {
            return pair.as_str().to_string();
        },

        // enmu类规则, 或者只有一条有效子规则的规则, 直接跳到 子规则
        | Rule::array_expr
        | Rule::branch_block
        | Rule::brackt_expr
        | Rule::expression
        | Rule::expression_except_chain
        | Rule::fn_def_identifier
        | Rule::if_block
        | Rule::literal_expr
        | Rule::loop_block
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
